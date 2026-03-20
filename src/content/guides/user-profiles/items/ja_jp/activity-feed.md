ユーザープロファイルのアクティビティフィードは、FastComments を導入しているすべてのサイトにおけるユーザーのコメント履歴、通知、およびコミュニティ参加状況を表示します。

### What is the Activity Feed?

アクティビティフィードは次の時系列ビューを提供します:
- ユーザーが投稿したコメント
- 受け取った返信
- メンションややり取り
- 異なるコミュニティ全体での活動
- ユーザーのエンゲージメントに関するリアルタイムの更新

### Viewing Activity

**Your Own Activity:**
自分のプロフィールを表示しているときは、追加のタブが表示されます:
1. **Notifications** - 返信、メンション、やり取りに関するアラート
2. **Recent Activity** - すべてのサイトにまたがるあなたの完全なコメント履歴

**Other Users' Activity:**
他のユーザーのプロフィールを表示しているとき:
1. **Recent Activity** - 公開されているそのユーザーのコメント履歴（非公開設定でない場合）

アクティビティタブは、ユーザーが FastComments ネットワークでどこでいつ参加しているかを示します。

### Notifications Tab

あなたの Notifications タブには次が表示されます:

**What You'll See:**
- あなたのコメントへの返信
- ユーザー名のメンション
- プロファイルコメントの通知
- ダイレクトメッセージのアラート
- バッジ獲得の通知
- コンテンツに対するモデレーションアクション

**Notification Features:**
- **Unread counter** - 未読通知の数を表示
- **Real-time updates** - 新しい通知は WebSocket 経由で即時表示されます
- **Clickable items** - 任意の通知をクリックして該当するコメントや会話にジャンプできます
- **Read/unread status** - どの通知を見たかの視覚的指標

**Managing Notifications:**
- 通知は表示したときに既読としてマークされます
- 受け取る通知の種類は [Notification Settings](https://fastcomments.com/auth/my-account/edit-notifications) で設定できます

詳細な通知管理については、[Notifications Guide](/guide-notifications.html) を参照してください。

### Recent Activity Tab

Recent Activity タブには、FastComments コミュニティ全体でユーザーが投稿したすべてのコメントが表示されます。

**What's Displayed:**
- **Comment content** - 各コメントの全文
- **Context** - コメントが付けられた記事／ページ
- **Timestamp** - コメントが投稿された日時
- **Community** - コメントが属するサイトやドメイン
- **Engagement** - 投票数、返信数、やり取りの指標

**Activity Filtering:**
自分のプロフィールでは、アクティビティを以下のようにフィルタできます:
- **All Activity** - あなたが投稿したすべてのコメント
- **Replies to Me** - あなたのコメントへの返信のみ

これにより、直接関与している会話に集中しやすくなります。

### Community Participation

プロファイルには、ユーザーがどのコミュニティで活動しているかも表示されます:

- **Community List** - ユーザーがコメントしたサイト／ドメインの一覧
- **Activity Distribution** - 各コミュニティでの参加度合い
- **Community Links** - それらのサイトに移動するためのクリック可能なリンク

これにより、ユーザーがどこで最も活発か、関心のある分野がどこかを理解できます。

### Statistics

ユーザープロファイルには主要な統計情報が表示されます:

**カルマ (Karma):**
- アップボートやコミュニティの評価に基づくレピュテーションスコア
- コメントがアップボートされると増加します
- コメントがダウンボートされると減少します
- あなたの貢献に対するコミュニティ全体の評価を反映します

**Comment Count:**
- すべてのコミュニティで投稿したコメントの合計数
- トップレベルコメントと返信の両方を含みます
- 参加に応じてリアルタイムで更新されます

これらの統計は、ユーザーのエンゲージメントレベルとコミュニティ内での立ち位置を素早く把握するのに役立ちます。

### Privacy Controls

ユーザーはプライバシー設定を通じてアクティビティの表示を制御できます:

**Make Profile Activity Private:**
有効にすると:
- Recent Activity タブはあなただけが閲覧できます
- 他のユーザーはプロフィール上のあなたのコメント履歴を閲覧できません
- あなたの公開コメントは元のコンテキスト（記事上）では引き続き表示されます
- プロファイル上の集約されたアクティビティビューのみが非表示になります

詳しくは [Privacy Settings](/guide-user-profiles.html#privacy-settings) を参照してください。

### Real-Time Updates

アクティビティフィードはリアルタイム更新のために WebSocket 技術を使用しています:

- **Instant notifications** - 新しい返信やメンションが即座に表示されます
- **Live activity updates** - 投稿すると最近のコメントが即時に追加されます
- **Online status** - 会話相手がアクティブかどうかを確認できます
- **No page refresh needed** - ページのリフレッシュは不要で、すべて自動的に更新されます

### Activity Across Communities

FastComments のユーザープロファイルの強力な機能の一つは、すべてのコミュニティにわたる活動を確認できることです:

**Benefits:**
- ユーザーがどこで知識があるか、どこで活発かを発見できる
- 異なるサイト間で共通の関心を持つユーザーを見つけられる
- FastComments ネットワーク全体で評判を築ける
- コミュニティ参加の幅広さを確認できる

**Examples:**
- テックフォーラムとゲームサイトで活発なユーザー
- ニュースとレシピブログの両方に寄稿する人
- 関連するトピックにまたがって参加するコミュニティメンバー

### Best Practices

**For Viewing Activity:**
- ユーザーのアクティビティを確認して、その専門性や興味を理解する
- コミュニティ参加状況を見て共通点を探す
- カルマとコメント数を見てエンゲージメントレベルを評価する

**For Managing Your Activity:**
- 自分の快適さに合ったプライバシー設定を行う
- アクティビティが表示されることを踏まえて慎重に参加する（非公開設定でない場合）
- フィルタを使って関連する会話に集中する
- 通知を追跡して会話に積極的に関与する

**For Community Building:**
- アクティビティフィードを通じて活発な参加者を認識する
- 興味を共有するユーザーとつながる
- 活動履歴を使って価値あるコミュニティメンバーを特定する

### Troubleshooting

**Activity Not Showing:**
- ユーザーが「Make Profile Activity Private」を有効にしていないか確認する
- 必要に応じてログインしているか確認する
- アクティビティが古いように見える場合はページを更新する

**Notifications Not Appearing:**
- アカウントの通知設定を確認する
- 通知がメールプロバイダによってフィルタされていないか確認する
- WebSocket 接続を確認する（リアルタイム更新の有無を確認）

**Wrong Activity Displayed:**
- ブラウザのキャッシュをクリアして更新する
- ログアウトして再ログインする
- 永続的な問題がある場合はサポートに報告する