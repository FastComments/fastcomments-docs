`NotificationCount` オブジェクトはユーザーの未読通知数とメタデータを表します。

未読の通知がない場合、ユーザーに対する `NotificationCount` は存在しません。

`NotificationCount` オブジェクトは自動的に作成され、API経由で作成することはできません。これらは1年で期限切れになります。

ユーザーの未読通知数は、その `NotificationCount` を削除することでクリアできます。

`NotificationCount` オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'NotificationCount の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // ユーザーID
    count: number
    createdAt: string // 日付文字列
    expireAt: string // 日付文字列
}
[inline-code-end]

---