FastCommentsでSAML認証を設定するには、管理ダッシュボードでの設定と、アイデンティティプロバイダー(IdP)側でのセットアップの両方が必要です。

### 前提条件

SAMLを設定する前に、以下を確認してください:

- FastCommentsのFlexまたはProプラン(SAMLはCreatorsプランでは利用できません)
- FastCommentsアカウントへの管理者アクセス権
- アイデンティティプロバイダーへの管理者アクセス権
- IdPのSAMLメタデータまたは証明書情報

### SAML設定へのアクセス

1. [FastComments管理ダッシュボード](https://fastcomments.com/auth/my-account)にログインします
2. 左側のサイドバーで **API/SSO Settings** に移動します
3. **SAML Config** ボタンをクリックします

SAML Configボタンが表示されない場合は、以下を確認してください:
- アカウントに必要なパッケージ(FlexまたはPro)が含まれていること
- 管理者権限を持っていること
- ユーザーにAPI AdminまたはAdmin Adminロールが割り当てられていること

### SAMLの基本設定

#### SAML認証の有効化

1. **Enable SAML Authentication** チェックボックスをオンにします
2. これにより、テナントでSAMLが有効になり、設定フィールドが利用可能になります

#### 必須フィールド

**IdP Single Sign-On URL** *(必須)*
- ユーザーが認証のためにリダイレクトされるURL
- 通常はアイデンティティプロバイダーから提供されます
- 例: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(必須)*
- アイデンティティプロバイダーの公開証明書
- SAMLレスポンスの真正性を検証するために使用されます
- BEGIN/ENDマーカーを含む完全な証明書を指定する必要があります
- 形式の例:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### 任意フィールド

**IdP Entity ID / Issuer**
- アイデンティティプロバイダーを識別します
- 空欄のままにした場合、FastCommentsのURLがデフォルトとして使用されます
- IdPで設定されたissuerと一致している必要があります

### 高度な設定

#### セキュリティ設定

**Signature Algorithm**
- デフォルトはSHA-256(推奨)
- 選択肢: SHA-1、SHA-256、SHA-512
- IdPの設定と一致している必要があります

**Digest Algorithm**
- デフォルトはSHA-256(推奨)
- SAMLレスポンスのダイジェスト計算に使用されます
- IdPの設定と一致している必要があります

**Name ID Format**
- デフォルトはメールアドレス形式
- ユーザー識別子のフォーマット方法を決定します
- 一般的な選択肢: メールアドレス、Persistent、Transient

#### 暗号化(任意)

**Private Key for Decryption**
- IdPがSAMLアサーションを暗号化する場合にのみ必要です
- 復号に使用する秘密鍵を貼り付けてください
- ほとんどの環境ではアサーションの暗号化は必要ありません

### 設定の保存

1. すべての設定が正確であることを確認します
2. **Save SAML Configuration** をクリックします
3. システムが設定を検証します
4. 成功すると、確認メッセージが表示されます

### 次のステップ

FastCommentsのSAML設定を保存した後:

1. Service Providerの情報を使用してアイデンティティプロバイダーを設定します
2. 認証フローをテストします
3. 必要に応じてユーザーロールと権限を設定します

SAMLを有効にすると、IdPの設定に必要なService Provider情報が表示されます。