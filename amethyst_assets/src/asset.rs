use std::ops::Deref;

use amethyst_error::Error;

use crate::processor::ProcessingState;

/// One of the three core traits of this crate.
///
/// You want to implement this for every type of asset like
///
/// * `Mesh`
/// * `Texture`
/// * `Terrain`
///
/// and so on. Now, an asset may be available in different formats.
/// That's why we have the `Data` associated type here. You can specify
/// an intermediate format here, like the vertex data for a mesh or the samples
/// for audio data.
///
/// This data is then generated by the `Format` trait.
pub trait Asset: Send + Sync + 'static {
    /// An identifier for this asset used for debugging.
    fn name() -> &'static str;

    /// The `Data` type the asset can be created from.
    type Data: Send + Sync + 'static;
}

/// Defines a way to process asset's data into the asset. This allows
/// using default `Processor` system to process assets that implement that type.
pub trait ProcessableAsset: Asset + Sized {
    /// Processes asset data into asset during loading.
    fn process(data: Self::Data) -> Result<ProcessingState<Self::Data, Self>, Error>;
}

impl<T: Asset<Data = T>> ProcessableAsset for T {
    fn process(data: Self::Data) -> Result<ProcessingState<Self::Data, Self>, Error> {
        Ok(ProcessingState::Loaded(data))
    }
}

/// A format, providing a conversion from bytes to asset data, which is then
/// in turn accepted by `Asset::from_data`. Examples for formats are
/// `Png`, `Obj` and `Wave`.
///
/// The format type itself represents loading options, which are passed to `import`.
/// E.g. for textures this would be stuff like mipmap levels and
/// sampler info.
pub trait Format<D: 'static>: objekt::Clone + Send + Sync + 'static {
    /// A unique identifier for this format.
    fn name(&self) -> &'static str;

    /// Produces asset data from given bytes.
    /// This method is a simplified version of `format`.
    /// This format assumes that the asset name is the full path and the asset is only
    /// contained in one file.
    ///
    /// If you are implementing `format` yourself, this method will never be used
    /// and can be left unimplemented.
    ///
    fn import_simple(&self, _bytes: Vec<u8>) -> Result<D, Error> {
        unimplemented!("You must implement either `import_simple` or `import`.")
    }
}

objekt::clone_trait_object!(<D> Format<D>);

/// SerializableFormat is a marker trait which is required for Format types that are supposed
/// to be serialized. This trait implies both `Serialize` and `Deserialize` implementation.
///
/// **Note:** This trait should never be implemented manually.
/// Use the `register_format` macro to register it correctly.
/// See [FormatRegisteredData](trait.FormatRegisteredData.html) for the full example.
pub trait SerializableFormat<D: 'static>: Format<D> + erased_serde::Serialize + 'static {
    // Empty.
}

// Allow using dynamic types on sites that accept format as generic.
impl<D: 'static> Format<D> for Box<dyn Format<D>> {
    fn name(&self) -> &'static str {
        self.deref().name()
    }
    fn import_simple(&self, bytes: Vec<u8>) -> Result<D, Error> {
        self.deref().import_simple(bytes)
    }
}

/// The `Ok` return value of `Format::import` for a given asset type `A`.
pub struct FormatValue<D> {
    /// The format data.
    pub data: D,
}

impl<D> FormatValue<D> {
    /// Creates a `FormatValue` from only the data.
    pub fn data(data: D) -> Self {
        FormatValue { data }
    }
}
