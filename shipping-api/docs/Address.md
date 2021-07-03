# Address

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the person, business or institution at that address. | [default to null]
**address_line1** | **String** | First line of that address. | [default to null]
**address_line2** | **String** | Additional address information, if required. | [optional] [default to null]
**address_line3** | **String** | Additional address information, if required. | [optional] [default to null]
**state_or_region** | [***::models::StateOrRegion**](StateOrRegion.md) |  | [default to null]
**city** | [***::models::City**](City.md) |  | [default to null]
**country_code** | [***::models::CountryCode**](CountryCode.md) |  | [default to null]
**postal_code** | [***::models::PostalCode**](PostalCode.md) |  | [default to null]
**email** | **String** | The email address of the contact associated with the address. | [optional] [default to null]
**copy_emails** | **Vec<String>** | The email cc addresses of the contact associated with the address. | [optional] [default to null]
**phone_number** | **String** | The phone number of the person, business or institution located at that address. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


