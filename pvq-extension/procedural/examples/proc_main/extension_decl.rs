use pvq_extension_procedural::extension_decl;

#[extension_decl]
pub mod extension_without_associated_type {
    #[extension_decl::extension]
    pub trait ExtensionWithoutAssociatedType {
        fn test_fn();
    }
}

#[extension_decl]
pub mod extension_core {
    #[extension_decl::extension]
    pub trait ExtensionCore {
        type ExtensionId;
        fn has_extension(id: Self::ExtensionId) -> bool;
    }
}

#[extension_decl]
pub mod extension_fungibles {
    #[extension_decl::extension]
    pub trait ExtensionFungibles {
        type AssetId;
        type AccountId;
        type Balance;
        fn total_supply(asset: Self::AssetId) -> Self::Balance;
        fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance;
    }
}
