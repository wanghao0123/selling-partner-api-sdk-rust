# ProductTypeDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta_schema** | [***::models::SchemaLink**](SchemaLink.md) | Link to meta-schema describing the vocabulary used by the product type schema. | [optional] [default to null]
**schema** | [***::models::SchemaLink**](SchemaLink.md) | Link to schema describing the attributes and requirements for the product type. | [default to null]
**requirements** | **String** | Name of the requirements set represented in this product type definition. | [default to null]
**requirements_enforced** | **String** | Identifies if the required attributes for a requirements set are enforced by the product type definition schema. Non-enforced requirements enable structural validation of individual attributes without all of the required attributes being present (such as for partial updates). | [default to null]
**property_groups** | [**::std::collections::HashMap<String, ::models::PropertyGroup>**](PropertyGroup.md) | Mapping of property group names to property groups. Property groups represent logical groupings of schema properties that can be used for display or informational purposes. | [default to null]
**locale** | **String** | Locale of the display elements contained in the product type definition. | [default to null]
**marketplace_ids** | **Vec<String>** | Amazon marketplace identifiers for which the product type definition is applicable. | [default to null]
**product_type** | **String** | The name of the Amazon product type that this product type definition applies to. | [default to null]
**product_type_version** | [***::models::ProductTypeVersion**](ProductTypeVersion.md) | The version details for the Amazon product type. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


