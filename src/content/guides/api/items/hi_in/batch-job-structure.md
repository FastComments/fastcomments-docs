एक `BatchJob` एक लंबे समय तक चलने वाला कार्य होता है, जैसे कोई import, export, या [page migration](#pages-migrations-post)। Jobs को queue में रखा जाता है, फिर background में उठाया और चलाया जाता है।

`processedCount` और `totalCount` प्रगति दिखाते हैं। इनकी इकाई `type` पर निर्भर करती है:

- `migrate-pages`: पूर्ण हुई migrations, अनुरोधित migrations में से।
- `undo-page-migration`: पुनर्स्थापित records। यहाँ कोई `totalCount` नहीं होता।
- `import`, `export` और अन्य jobs: प्रोसेस की गई rows।

`BatchJob` object की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'BatchJob संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** उदाहरण के लिए migrate-pages, undo-page-migration, migrate-domains, import, export, users-export या copy-comments। **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** कुल कार्य, जब यह पहले से ज्ञात हो। **/
    totalCount?: number
    /** migrate-pages: प्रत्येक पूर्ण हुई migration के लिए एक entry, अनुरोध के क्रम में। **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: job पूर्ण होने पर (success या failed) एक बार सेट होता है। 90 दिनों के भीतर POST /api/v1/pages/migrations/:undoId/undo को पास करें।
     * undo-page-migration: जो migration पूर्ववत की जा रही है।
     **/
    undoId?: string
    /** undo-page-migration: job पूर्ण होने पर सेट होता है। **/
    pageMigrationUndoResult?: PageMigrationUndoResult
}

interface PageMigrationSuccess {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'success'
    commentsMigrated: number
}

interface PageMigrationFailure {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'failed'
    code: string
    reason: string
}

interface PageMigrationUndoResult {
    /** Records जिन्हें migration से पहले के मानों पर वापस रखा गया। **/
    restored: number
    /** Records जिन्हें छोड़ दिया गया क्योंकि वे migration के बाद बदल गए थे, या अब मौजूद नहीं हैं। **/
    skipped: number
    /** Records जिन्हें migration ने pages merge करते समय delete किया। इन्हें वापस लाया नहीं जा सकता। **/
    notRestorable: number
}
[inline-code-end]

#### Retries

यदि page migration job चलाने वाला server restart हो जाता है, तो job वापस queue में चला जाता है और अंतिम पूर्ण हुई migration के बाद से फिर शुरू होता है। एक job को अधिकतम 3 बार try किया जाता है, जिसके बाद उसका `status` `failed` हो जाता है।