#### Sakai

Sakai は LTI Advantage を含むリリースで LTI 1.3 の Dynamic Registration をサポートしています。Administration Workspace から:

1. Sakai の管理者としてサインインし、**Administration Workspace** を開きます。
2. **External Tools** > **Install LTI 1.3 Tool** を選択します。
3. FastComments の登録用 URL（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ここで取得</a>）を貼り付けて送信します。
4. ハンドシェイクが完了したらツールを承認します。

ツールはその後 **External Tools** に表示され、サイトの管理者によってサイトに追加できます。

#### Schoology

Schoology の Enterprise インスタンスは LTI 1.3 をサポートしますが、Dynamic Registration の利用可否はデプロイによって異なります。Schoology のアカウントマネージャーに確認してください。

お使いの Schoology インスタンスで Dynamic Registration が利用できない場合、次のエンドポイントを使って統合を手動で設定する必要があります:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Schoology から Client ID と Deployment ID が提供されたら、テナント上で設定を登録するために FastComments サポートへ連絡してください。

#### Other LTI 1.3 Platforms

IMS LTI 1.3 Advantage 仕様に準拠する任意の LMS は、同じ登録 URL（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ここで取得</a>）で動作するはずです。設定項目として "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", またはそれに類する項目を探してください。

プラットフォームが手動での LTI 1.3 設定のみをサポートする場合は、上の Schoology セクションに記載された 4 つのエンドポイントを使用し、設定完了のためにサポートに連絡してください。