FastComments は SAML のユーザーロールを内部の権限にマッピングし、組織向けのロールベースのアクセス制御を可能にします。

### FastComments のロールシステム

FastComments はロールベースの権限システムを使用しており、ユーザーは1つ以上のロールを持て、そのロールがアクセスレベルや操作権限を決定します。

### 利用可能な FastComments ロール

#### 管理ロール

**fc-account-owner**
- **Permissions**: 完全な管理アクセス
- **Capabilities**: すべての機能、請求管理、ユーザー管理
- **Use Case**: 主要なアカウント管理者および所有者

**fc-admin-admin**  
- **Permissions**: ほとんどの機能への管理アクセス
- **Capabilities**: ユーザー管理、設定、モデレーション。 **他の管理者を管理できます。**
- **Use Case**: 二次的な管理者およびITスタッフ

**fc-billing-admin**
- **Permissions**: 請求およびサブスクリプション管理
- **Capabilities**: 支払い方法、請求書、サブスクリプションの変更
- **Use Case**: 財務チームのメンバーおよび請求担当者

#### 専門ロール

**fc-analytics-admin**
- **Permissions**: 分析およびレポートへのアクセス
- **Capabilities**: サイト統計、ユーザーエンゲージメントデータの閲覧
- **Use Case**: マーケティングチームおよびデータアナリスト

**fc-api-admin**
- **Permissions**: API へのアクセスと管理
- **Capabilities**: API 資格情報、webhook の構成
- **Use Case**: 開発者および技術統合担当者

**fc-moderator**
- **Permissions**: コメントのモデレーション権限
- **Capabilities**: コメントの承認/却下、スパム管理
- **Use Case**: コミュニティモデレーターおよびコンテンツマネージャー

### ロールマッピングの設定

#### SAML 属性のソース

FastComments は、異なるアイデンティティプロバイダーとの互換性を確保するため、さまざまな SAML 属性名からロール情報を受け取ります：

**Standard Attribute Names**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS Attributes**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### ロール形式のサポート

**Array Format** *(Preferred)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Comma-Separated Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Single Role Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### アイデンティティプロバイダーのロール設定

#### Microsoft Azure AD

1. **アプリロールの構成**:
   - Azure AD アプリケーション内で FastComments のロールを定義する
   - ユーザーを適切なアプリロールに割り当てる
   - claims を構成して割り当てられたロールを含める

2. **Attribute Mapping**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Group Assignment**:
   - FastComments のロール名と一致するグループを作成する
   - ユーザーを適切なグループに割り当てる
   - 属性ステートメントを構成する

2. **Attribute Statement**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Group Mapping**:
   - 組織単位またはグループを作成する
   - グループ名に FastComments ロールのプレフィックスを付ける
   - 属性マッピングを構成する

2. **Custom Attributes**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### デフォルトのユーザーの挙動

#### ロールを持たないユーザー

SAML ユーザーにロールがない、または認識されないロールしかない場合：
- ユーザーは標準のコメントユーザーとして作成される
- 管理者アクセスは付与されない
- 自分のコメントを投稿および管理できる
- 管理ダッシュボード機能にはアクセスできない

#### ロール継承

- ユーザーは同時に複数のロールを持てる
- 権限は累積される（最も高い権限レベルが適用される）
- IdP 側のロール変更は次回ログイン時に反映される

### SAML ユーザーの管理

#### ユーザー作成

ユーザーが初めて SAML でログインしたとき：
1. **ユーザーアカウント**: メールを識別子として自動作成される
2. **ロール割り当て**: SAML 属性に基づいてロールが適用される
3. **プロファイル情報**: 提供されていれば名／姓が設定される
4. **権限の有効化**: ロールは即時に有効になる

#### ロール更新

既存の SAML ユーザーはロールの更新を受ける：
1. **ログイントリガー**: ロール更新は各 SAML ログイン時に発生する
2. **即時反映**: 新しい権限は即時に適用される
3. **ロール削除**: 削除されたロールは自動的に取り消される
4. **監査ログ**: ロール変更は監査ログに記録される

### カスタムロールマッピング

#### エンタープライズ向けカスタマイズ

特定の要件を持つエンタープライズのお客様向け：
- カスタムロール名を FastComments の権限にマッピングできる
- 複雑なロール階層を実装できる
- 部署別のアクセス制御を構成できる

カスタムロールマッピングの設定については FastComments サポートにお問い合わせください。

#### ロールの検証

FastComments は受信したロールを検証します：
- 認識されないロールは無視される（拒否はされない）
- 不正な形式のロール属性はトラブルシューティングのためにログに記録される
- SAML アサーションにロール情報がない場合、ユーザーは既存のロールを保持する

### ベストプラクティス

#### ロール管理

1. **最小権限の原則**: 必要最小限の権限を割り当てる
2. **定期的な監査**: ユーザーロールとアクセスを定期的に見直す  
3. **明確な命名**: IdP 内で説明的なグループ名を使用する
4. **ドキュメンテーション**: ロール割り当てのドキュメントを維持する

#### セキュリティに関する考慮事項

1. **ロール属性**: SAML レスポンス内のロール属性が適切に保護されていることを確認する
2. **属性の検証**: 認可されたシステムのみがロールを割り当てられることを確認する
3. **アクセスレビュー**: 管理者ロールの割り当てを定期的に見直す
4. **監視**: ロール変更と管理者アクションを監視する

### ロール関連のトラブルシューティング

#### よくある問題

**ロールが適用されない場合**:
- SAML 属性名がサポートされている形式と一致しているか確認する
- IdP がロール情報を送信しているか確認する
- ロール値が FastComments のロール名と正確に一致していることを確認する

**アクセス拒否**:
- ユーザーに IdP で適切なロールが割り当てられているか確認する
- ロールのスペルと大文字小文字が正しいか確認する
- SAML レスポンス内でロールが正しくフォーマットされていることを確認する

**権限が不足している場合**:
- ロール定義と必要な権限を確認する
- 競合するロール割り当てがないか確認する
- ロール変更後にユーザーがログインしていることを確認する