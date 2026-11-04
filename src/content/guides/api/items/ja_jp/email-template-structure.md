`EmailTemplate` オブジェクトは、テナント向けのカスタムメールテンプレートの設定を表します。

システムは以下の方法で使用するメールテンプレートを選択します:

- そのタイプ識別子。これを `emailTemplateId` と呼びます。これらは定数です。
- `domain`。まず、関連するオブジェクト(例えば `Comment`)が紐づいているドメインのテンプレートを探し、一致が見つからない場合は、domain が null または `*` のテンプレートを探します。

`EmailTemplate` オブジェクトの構造は以下の通りです:

[inline-code-attrs-start title = 'メールテンプレートの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** READONLY **/
    createdAt: string
    /** READONLY **/
    updatedAt: string
    /** READONLY **/
    updatedByUserId: string
    /** The domain the template should be associated with. **/
    domain?: string | '*' | null
    /** The email template content in EJS syntax. **/
    ejs: string
    /** A map of overridden translation keys to values, for each supported locale. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** An object that represents the render context of the template. **/
    testData: object
}
[inline-code-end]

### 注意事項

- 有効な `emailTemplateId` の値は `/definitions` エンドポイントから取得できます。
- `/definitions` エンドポイントには、デフォルトの翻訳とテストデータも含まれています。
- 構造やテストデータが無効な場合、テンプレートは保存に失敗します。