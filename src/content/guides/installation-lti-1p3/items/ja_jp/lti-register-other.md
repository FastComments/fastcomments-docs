#### Sakai

Sakai は LTI Advantage を搭載したリリースで LTI 1.3 Dynamic Registration をサポートしています。管理ワークスペースから:

1. Sakai の管理者としてサインインし、**管理ワークスペース**を開きます。
2. **外部ツール** > **LTI 1.3 ツールをインストール** を選択します。
3. FastComments の登録用 URL を貼り付けて送信します。
4. ハンドシェイクが完了したらツールを承認します。

そのツールは **外部ツール** の下に表示され、サイトの管理者がサイトに追加できます。

#### Schoology

Schoology Enterprise インスタンスは LTI 1.3 をサポートしますが、Dynamic Registration の利用可否はデプロイによって異なります。Schoology のアカウントマネージャーに確認してください。

もし Schoology インスタンスで Dynamic Registration が利用できない場合は、次のエンドポイントを使用して手動で統合を設定する必要があります:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Schoology から Client ID と Deployment ID が提供されたら、FastComments サポートに連絡してテナント上に設定を登録してください。

#### Other LTI 1.3 Platforms

IMS LTI 1.3 Advantage 仕様に準拠している LMS であれば、同じ登録 URL で動作するはずです。"Dynamic Registration"、"Tool Registration URL"、"Tool initiation registration endpoint" といった名称の設定を探してください。

プラットフォームが手動の LTI 1.3 設定のみをサポートしている場合は、上記 Schoology セクションに記載されている4つのエンドポイントを使用し、設定を完了するためにサポートに連絡してください。