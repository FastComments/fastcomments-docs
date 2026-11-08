## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| skip | integer | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |
| contextUserId | string | query | नहीं |  |
| hashTag | string | query | नहीं |  |
| parentId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| fromDate | integer | query | नहीं |  |
| toDate | integer | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: `APIGetCommentsResponse`

## उदाहरण

[inline-code-attrs-start title = 'getComments उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे की लाइन अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final page = 56; // int | 
final limit = 56; // int | 
final skip = 56; // int | 
final asTree = true; // bool | 
final skipChildren = 56; // int | 
final limitChildren = 56; // int | 
final maxTreeDepth = 56; // int | 
final urlId = urlId_example; // String | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 
final contextUserId = contextUserId_example; // String | 
final hashTag = hashTag_example; // String | 
final parentId = parentId_example; // String | 
final direction = ; // SortDirections | 
final fromDate = 789; // int | 
final toDate = 789; // int | 

try {
    final result = api_instance.getComments(tenantId, GetCommentsOptions(page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction, fromDate: fromDate, toDate: toDate));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getComments: $e\n');
}
[inline-code-end]