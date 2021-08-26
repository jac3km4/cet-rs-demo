use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::Fields::Named;
use syn::{
    self, parse_macro_input, Attribute, Block, Field, FnArg, ImplItem, ImplItemMethod, ItemImpl, ItemStruct, Pat, Type
};

#[proc_macro_attribute]
pub fn vft_class(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item: ItemStruct = parse_macro_input!(item as ItemStruct);

    // insert the vft pointer
    if let Named(fields) = &mut item.fields {
        let field = Field::parse_named.parse2(quote! { vft: *const *const usize }).unwrap();
        fields.named.insert(0, field);
    } else {
        panic!("You can't use #[cpp_class] with unnamed structs");
    };

    let class_impl = impl_cpp_class(&item);
    item.attrs
        .extend(Attribute::parse_outer.parse2(quote! {#[repr(C)]}).unwrap());

    let mut stream = item.into_token_stream();
    stream.extend(class_impl);
    stream.into()
}

#[proc_macro_attribute]
pub fn vft_class_extending(attr: TokenStream, body: TokenStream) -> TokenStream {
    let base_ty = parse_macro_input!(attr as Type);
    let mut item: ItemStruct = parse_macro_input!(body as ItemStruct);

    // insert a base class instance
    if let Named(fields) = &mut item.fields {
        let field = Field::parse_named.parse2(quote! { pub base: #base_ty }).unwrap();
        fields.named.insert(0, field);
    } else {
        panic!("You can't use #[cpp_class] with unnamed structs");
    };

    let class_impl = impl_cpp_class_with_base(&item, &base_ty);
    item.attrs
        .extend(Attribute::parse_outer.parse2(quote! {#[repr(C)]}).unwrap());

    let mut stream = item.into_token_stream();
    stream.extend(class_impl);
    stream.into()
}

#[proc_macro_attribute]
pub fn vft_class_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item: ItemImpl = parse_macro_input!(item as ItemImpl);
    let mut index = 0usize;

    // implement virtual calls
    for member in &mut item.items {
        if let ImplItem::Method(method) = member {
            if method.attrs.iter().any(|attr| attr.path.is_ident("virt")) {
                method.block = Block::parse.parse2(impl_virt_call(method, index)).unwrap();
                index += 1;
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let self_ty = item.self_ty.as_ref();

    let impl_instance = quote! {
        impl #impl_generics CppClassMethods for #self_ty #ty_generics #where_clause {
            const VTABLE_SIZE: usize = #index;
        }
    };

    let mut stream = item.into_token_stream();
    stream.extend(impl_instance);
    stream.into()
}

#[proc_macro_attribute]
pub fn virt(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

fn impl_cpp_class(item: &ItemStruct) -> proc_macro2::TokenStream {
    let ident = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        impl #impl_generics CppClass for #ident #ty_generics #where_clause {
            #[inline(always)]
            unsafe fn lookup_method<FN>(&self, idx: usize) -> FN {
                let vft = self.vft as *const FN;
                vft.add(idx).read()
            }
        }
    }
}

fn impl_cpp_class_with_base(item: &ItemStruct, base: &Type) -> proc_macro2::TokenStream {
    let ident = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        impl #impl_generics CppClass for #ident #ty_generics #where_clause {
            #[inline(always)]
            unsafe fn lookup_method<FN>(&self, idx: usize) -> FN {
                const offset: usize = <#base as CppClassMethods>::VTABLE_SIZE;
                self.base.lookup_method(idx + offset)
            }
        }
    }
}

fn impl_virt_call(method: &ImplItemMethod, index: usize) -> proc_macro2::TokenStream {
    let sig = &method.sig;
    let ret_ty = &sig.output;
    let args = sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(pat) = arg {
            if let Pat::Ident(ident) = pat.pat.as_ref() {
                return Some((ident.ident.clone(), pat.ty.as_ref()));
            }
        }
        None
    });
    let arg_tys = args.clone().map(|(_, ty)| ty);
    let arg_ids = args.map(|(id, _)| id);

    quote! {
        { unsafe { self.lookup_method::<fn(*const Self, #(#arg_tys),*) #ret_ty>(#index)(self, #(#arg_ids),*) } }
    }
}
