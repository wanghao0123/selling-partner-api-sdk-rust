# PatchOperation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**op** | **String** | Type of JSON Patch operation. Supported JSON Patch operations include add, replace, and delete. See &lt;https://tools.ietf.org/html/rfc6902&gt;. | [default to null]
**path** | **String** | JSON Pointer path of the element to patch. See &lt;https://tools.ietf.org/html/rfc6902&gt;. | [default to null]
**value** | [**Vec<Value>**](Value.md) | JSON value to add, replace, or delete. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


