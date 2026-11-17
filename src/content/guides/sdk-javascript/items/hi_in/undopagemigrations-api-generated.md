एक बैच जॉब कतार में डालता है जो एक migrate-pages जॉब के हर माइग्रेशन को उलट देता है, नवीनतम पहले। 90 दिनों तक उपलब्ध।
माइग्रेशन के बाद बदले गए रिकॉर्ड को नहीं छेड़ा जाता, इसलिए बाद का संपादन कभी अधिलेखित नहीं होता।
बहाल/छोड़े गए की गिनती के लिए GET /api/v1/batch-jobs/{batchJobId} को पोल करें।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| undoId | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'undoPageMigrations उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]