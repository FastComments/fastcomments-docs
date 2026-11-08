एक urlId या domain से दूसरे में कमेंट्स, पेज और संबंधित रिकॉर्ड्स को स्थानांतरित करने वाली बैच जॉब कतार में डालता है।
"urlId" माइग्रेशन केवल सटीक urlId से मेल खाते हैं। "domain" माइग्रेशन हर urlId और url के भीतर मान को खोजते/बदलते हैं।
GET /api/v1/batch-jobs/{batchJobId} को पोल करें: एक बार पूरा होने पर इसमें प्रति-माइग्रेशन परिणाम और एक undoId होता है।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'migratePages उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let migrateAPIPagesData = MigrateAPIPagesData(migrations: [PageMigration(from: "from_example", to: "to_example", type: PageMigrationType())]) // MigrateAPIPagesData | 

DefaultAPI.migratePages(tenantId: tenantId, migrateAPIPagesData: migrateAPIPagesData) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]