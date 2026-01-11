ユーザープロフィールがあなたのサイトのコンテキスト（コメントウィジェット経由）で開かれると、FastCommentsウィジェットに適用したカスタムCSSスタイルは自動的にプロフィールモーダルに注入されます。

### 仕組み

ユーザーがコメントウィジェットのプロフィールリンクをクリックすると、クラス `.fast-comments-profile` を持つプロフィールモーダルが開きます。ウィジェットに設定したカスタムCSSはプロフィール表示に自動的に注入されます。すでにコメントウィジェットをスタイルしている場合、そのスタイルはプロフィールにも適用されます。

### CSSクラス

FastCommentsのプロフィールはクラスベースのCSSアーキテクチャを使用しています。CSSカスタムプロパティは使用していません。

メインのプロフィールページはルートコンテナとして `.user-profile` を使用します。ヘッダーセクションは `.profile-header` で、背景画像には `.profile-header-background` を使用します。プロフィールのコンテンツは `.profile-content` に配置されます。

アバターは `.profile-avatar` と `.profile-avatar-wrapper` を使用します。ユーザー名は `.profile-name`、自己紹介文は `.profile-bio` です。統計情報は `.profile-stats` にあり、各統計は `.stat` を使用します。

ソーシャルリンクは `.profile-social-links` にあり、個々のリンクは `.social-link` です。バッジは `.profile-badges` と `.badge` を使用します。バッジのプログレスバーは `.progress-outer` と `.progress-bar` を使用します。

タブはコンテナに `.profile-tabs`、個々のタブに `.tab`、選択されたタブには `.tab.active` を使用します。タブのコンテンツは `.tab-body` と `.tab-body.active` を使用します。タブ上の通知数は `.tab .count` を使用します。

通知は `.notification` を使用し、DM会話は `.conversation` を使用します。オンライン状態は `.activity-indicator` で表し、アクティブ状態は `.activity-indicator.online` です。未読カウンターは `.unread-count` を使用します。

プロフィールモーダルのコンテナは `.fast-comments-profile` で、閉じるボタンは `.fast-comments-profile-close` です。

### ダークモード

ダークモードは `.user-profile` に対する `.dark` クラス修飾子を使用します。

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### 例

**ヘッダー:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**バッジ:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**タブ:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**モーダル:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```