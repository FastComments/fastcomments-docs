FastCommentsでSAMLを設定した後、IdP（アイデンティティプロバイダ）側でFastCommentsをサービスプロバイダとして設定する必要があります。

### 一般的な IdP の設定

ほとんどのアイデンティティプロバイダは、FastCommentsをSAMLアプリケーションとして追加するために次の情報を要求します。

#### 必須のサービスプロバイダ情報

これらの値は自動的に生成され、FastCommentsのSAML設定ページに表示されます:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- これはあなたのFastCommentsインスタンスを一意に識別します

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- 認証後にIdPがSAMLレスポンスを送信する場所

**SP Metadata URL** *(if supported by your IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- XML形式で完全なSAML構成を提供します

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- SAML認証を開始するための直接リンク

### 必要なSAML属性

アイデンティティプロバイダがSAMLレスポンスと共にこれらの属性を送信するように構成してください:

#### 必須属性

**Email Address** *(Required)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Purpose**: ユーザーの一意識別と通知
- **Format**: 有効なメールアドレス

#### オプション属性

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Purpose**: ユーザー表示名

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Purpose**: ユーザー表示名

**Roles** *(Important for access control)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Purpose**: FastCommentsのロール割り当てと権限
- **Format**: ロール文字列の配列またはカンマ区切りの値

### 一般的なアイデンティティプロバイダの構成例

#### Microsoft Azure AD

1. **Add Enterprise Application**
   - "FastComments"を検索するか、カスタムSAMLアプリを作成します
   - FastCommentsが提供するSP情報を使用します

2. **Configure Attributes**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Create SAML Application**
   - "Create New App"を使用し、SAML 2.0を選択します
   - FastCommentsのSP情報で構成します

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Add SAML Application**
   - Apps > Web and mobile apps > Add App > Add custom SAML app に移動します
   - FastCommentsのSP情報で構成します

2. **Attribute Mapping**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Add Relying Party Trust**
   - FastCommentsのmetadata URLを使用するか手動で構成します
   - 提供されたSP情報を構成します

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: グループメンバーシップまたはカスタムクレーム

### 属性名の柔軟性

FastCommentsは、さまざまなIdP構成に対応するために複数の属性名からロール情報を受け取ります:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

この柔軟性により、特定の属性命名規約を要求せずに、さまざまなアイデンティティプロバイダと互換性を確保します。

### 設定のテスト

IdPを構成した後:

1. IdPの設定を保存します
2. 専用のテストユーザーアカウントでテストします
3. 属性が正しく送信されていることを確認します
4. ロールが正しくマッピングされていることを確認します
5. 認証フローが正常に完了することを確認します

ほとんどのアイデンティティプロバイダは、本番ユーザーに展開する前に構成を検証するためのSAMLテストツールを提供しています。