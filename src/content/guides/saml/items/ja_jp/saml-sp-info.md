When SAML が FastComments で有効になっている場合、システムは ID プロバイダに設定するために必要な Service Provider (SP) 情報を自動的に生成します。

### Accessing Service Provider Information

SP 情報は SAML 認証を有効にした後、SAML 設定ページに表示されます。この情報には、SAML トラスト関係を確立するために ID プロバイダが必要とするすべての詳細が含まれます。

### Service Provider Endpoints

#### SP Entity ID / Audience
**Purpose**: FastComments インスタンスをサービスプロバイダとして一意に識別します  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Usage**: これを IdP の Entity ID または Audience として設定してください

この識別子は、SAML レスポンスが特定の FastComments テナント宛てであることを保証し、他のインスタンスによる SAML レスポンスの受け入れを防ぎます。

#### Assertion Consumer Service (ACS) URL
**Purpose**: ユーザ認証後に IdP が SAML レスポンスを送信するエンドポイント  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Usage**: これを IdP の ACS URL または Reply URL として設定してください

これは、ユーザが ID プロバイダでの認証に成功した後にリダイレクトされる場所であり、ユーザ情報を含む SAML アサーションが送信されます。

#### SP Metadata URL
**Purpose**: 標準の XML 形式で完全な SAML 設定を提供します  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Usage**: 一部の IdP はこの URL を使って構成を自動インポートできます

メタデータ URL には XML 形式で必要な SP 情報がすべて含まれており、互換性のある ID プロバイダを自動的に設定するのが簡単になります。

#### SAML Login URL
**Purpose**: テナントの SAML 認証を開始するための直接リンク  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Usage**: ユーザを直接 SAML 認証に誘導するか、フローのテストに使用します

この URL は SAML 認証をテストするため、またはユーザに SAML 経由でサインインする直接リンクを提供するために使用できます。

### SAML Binding Support

FastComments は次の SAML バインディングをサポートしています:

#### HTTP-POST Binding
- **Primary Method**: SAML レスポンスで最も一般的なバインディングです  
- **Security**: SAML レスポンスは HTTP POST を介して ACS URL に送信されます  
- **Usage**: 本番環境での展開には推奨されます

#### HTTP-Redirect Binding
- **Alternative Method**: SAML レスポンスが HTTP リダイレクトを介して送信されます  
- **Limitations**: URL 長の制限によりペイロードサイズが制限されます  
- **Usage**: サポートされていますが HTTP-POST が推奨されます

### Name ID Policy

FastComments は SAML リクエストで次の Name ID ポリシーを設定します:

- **Default Format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternative Formats**: Persistent、Transient、Unspecified（設定可能）  
- **Requirement**: メールアドレスが主要なユーザ識別子として使用されます

### SAML Request Attributes

SAML 認証を開始する際、FastComments は次の特徴をもつリクエストを送信します:

#### Request Signing
- **Status**: 任意（設定可能）  
- **Algorithm**: 設定された署名アルゴリズムに合わせます  
- **Certificate**: リクエスト署名が有効な場合はテナント固有の証明書を使用します

#### Requested Attributes
FastComments は SAML AuthnRequests で次の属性を要求します:

- **Email**: ユーザ識別のために必須  
- **First Name**: 表示目的で任意  
- **Last Name**: 表示目的で任意  
- **Roles/Groups**: アクセス制御や権限のために任意

### Copying SP Information

SAML 設定ページには、SP 情報をクリップボードに自動コピーするクリック可能なフィールドが用意されています:

1. 任意の SP 情報フィールド（Entity ID、ACS URL など）をクリックします  
2. 値が自動的にクリップボードにコピーされます  
3. 値を ID プロバイダの設定に貼り付けます  
4. コピーが成功すると短時間ハイライト表示されます

これにより、SP 情報を手入力によるミスなしに正確に IdP に転送できます。

### SP Certificate Information

#### Certificate Usage
- **Purpose**: 通信を暗号化し、SP の身元を検証するため  
- **Rotation**: 証明書は FastComments によって自動的に管理されます  
- **Access**: 公開証明書はメタデータ URL から入手できます

#### Certificate Details
- **Algorithm**: RSA-2048 またはそれ以上  
- **Validity**: 証明書は期限切れ前に自動的に更新されます  
- **Distribution**: 標準の SAML メタデータを通じて入手可能です

### Troubleshooting SP Configuration

ID プロバイダが SP 情報に関して問題を報告する場合:

1. **Verify URLs**: すべての URL が HTTPS を使用し、正しいテナント ID を含んでいることを確認してください  
2. **Check Metadata**: メタデータ URL を使用して構成を検証してください  
3. **Test Connectivity**: IdP が FastComments のエンドポイントに到達できることを確認してください  
4. **Validate Format**: IdP が SP 情報の形式をサポートしていることを確認してください

一般的な問題には次のようなものがあります:
- URL に誤ったテナント ID が含まれている  
- IdP と FastComments 間のネットワーク接続の問題  
- IdP が異なる URL 形式や追加の構成オプションを期待している