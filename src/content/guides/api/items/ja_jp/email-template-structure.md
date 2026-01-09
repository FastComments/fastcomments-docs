An `EmailTemplate` object represents configuration for a custom email template, for a tenant.

The system will select the email template to use via:

- Its type identifier, we call this `emailTemplateId`. These are constants.
- The `domain`. We will first try to find a template for the domain that the related object (like a `Comment`) is tied to, and if a match is not found then we will try to find a template where domain is null or `*`.

The structure for the `EmailTemplate` object is as follows:

[inline-code-attrs-start title = 'メールテンプレートの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** 読み取り専用 **/
    createdAt: string
    /** 読み取り専用 **/
    updatedAt: string
    /** 読み取り専用 **/
    updatedByUserId: string
    /** テンプレートが関連付けられるドメイン。 **/
    domain?: string | '*' | null
    /** EJS 構文で記述されたメールテンプレートの内容。 **/
    ejs: string
    /** サポートされている各ロケールごとの、上書きされた翻訳キーから値へのマップ。 **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** テンプレートのレンダリングコンテキストを表すオブジェクト。 **/
    testData: object
}
[inline-code-end]

### 注意

- You can get the valid `emailTemplateId` values from the `/definitions` endpoint.
- The `/definitions` endpoint also includes the default translations and test data.
- Templates will fail to save with invalid structure or test data.