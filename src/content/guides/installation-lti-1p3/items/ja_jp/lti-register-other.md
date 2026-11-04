#### Sakai

Sakaiは、LTI Advantageを備えたリリースにおいてLTI 1.3 Dynamic Registrationをサポートしています。Administration Workspaceから:

1. Sakai管理者としてサインインし、**Administration Workspace**を開きます。
2. **External Tools** > **Install LTI 1.3 Tool**を選択します。
3. FastCommentsの登録URL(<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">こちらから取得</a>)を貼り付けて送信します。
4. ハンドシェイクが完了したら、ツールを承認します。

その後、ツールは**External Tools**の下に表示され、サイトの管理者がサイトに追加できるようになります。

#### Schoology

Schoology EnterpriseインスタンスはLTI 1.3をサポートしていますが、Dynamic Registrationの利用可否はデプロイメントによって異なります。Schoologyのアカウントマネージャーにご確認ください。

お使いのSchoologyインスタンスでDynamic Registrationが利用できない場合は、以下のエンドポイントを使用して統合を手動で設定する必要があります:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

SchoologyからClient IDとDeployment IDが発行されたら、FastCommentsサポートに連絡して、テナントに設定を登録してください。

#### その他のLTI 1.3プラットフォーム

IMS LTI 1.3 Advantage仕様に準拠したLMSであれば、同じ登録URL(<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">こちらから取得</a>)で動作するはずです。「Dynamic Registration」、「Tool Registration URL」、「Tool initiation registration endpoint」などのラベルが付いた設定を探してください。

お使いのプラットフォームがLTI 1.3の手動設定のみをサポートしている場合は、上記のSchoologyセクションに記載されている4つのエンドポイントを使用し、サポートに連絡して設定を完了してください。