एक urlId या डोमेन से दूसरे urlId या डोमेन पर कमेंट्स, पेज और संबंधित रिकॉर्ड्स को स्थानांतरित करने वाली बैच जॉब को क्यू (queue) करता है।
"urlId" माइग्रेशन केवल सटीक urlId से मैच करते हैं। "domain" माइग्रेशन हर urlId और url के अंदर मान को खोजने/बदलने (find/replace) का काम करते हैं।
GET /api/v1/batch-jobs/{batchJobId} को पोल (poll) करें: जैसे ही यह पूरा होता है, इसमें प्रति-माइग्रेशन परिणाम और एक undoId होता है।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| migrateAPIPagesData | MigrateAPIPagesData | हाँ |  |

## प्रतिक्रिया (Response)

रिटर्न करता है: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'migratePages उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';

const migrations: PageMigration[] = [
  {
    type: PageMigrationType.FULL_MIGRATION,
    url: 'https://myblog.com/posts/react-hooks-guide',
    title: 'The Complete Guide to React Hooks',
    comments: 'disqus-comments-export.xml',
    date: new Date('2024-01-15T10:30:00Z')
  },
  {
    type: PageMigrationType.COMMENT_ONLY,
    url: 'https://myblog.com/posts/typescript-tips',
    title: '10 Essential TypeScript Tips',
    comments: [
      { id: 'c-101', author: 'jane_doe', message: 'Great article, thanks for sharing!', date: '2024-01-16T08:12:00Z' },
      { id: 'c-102', author: 'dev_mike', message: 'Very helpful, bookmarked this one.', date: '2024-01-16T14:47:00Z' }
    ]
  }
];

const migrateAPIPagesData: MigrateAPIPagesData = {
  source: 'disqus',
  dryRun: false,
  pages: migrations
};

const response: QueuePageMigrationJobAPIResponse = await migratePages(tenantId, migrateAPIPagesData);

if (response.status === 'success') {
  console.log(`Queued migration job: ${response.jobId} for ${migrateAPIPagesData.pages.length} pages`);
}
[inline-code-end]