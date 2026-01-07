### フォーマット例

スケジュールコメントのCSVは次のようになります:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### フォーマットの詳細

スケジュールコメントCSVファイルは次の列をサポートしています:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Hours
- Minutes
- Seconds

次の列は**必須**です:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

自動化されたネストされた返信をサポートしたい場合は、Parent ID列が必要です。

### フォーマットの仕組み

インポートフォーマットは次のように動作します:

1. 投稿したい各コメントに対してCSVに行を定義します。
2. コメントは指定された遅延（時間 + 分 + 秒）後に投稿されます。
   1. 手動でスケジュールされたインポートの場合、遅延はインポートが完了した後にUIで「play」を押した時点からの相対的なものです - **インポートが開始した時点ではありません**。
   2. 自動スケジュールインポートの場合、遅延はページ読み込み時からです。
3. IDを定義する必要があります。1、2、3、4、5のような増分識別子を使用できます。
4. ネストされた返信を使用したい場合は、`Parent ID`列の値を別のコメントの`ID`に設定するだけです。
5. `Comment`フィールドは、FastCommentsがコメントウィジェットでテキストのスタイリングや画像の追加のためにサポートしているのと同じ構文をサポートしています。
6. `Avatar`フィールドを使用する場合は、公開アクセス可能な画像である必要があります。当社のCDNにコピーされ、そこから配信されます。
7. 再生後、または再生が停止した場合、すべてのコメントを削除できます。
8. 削除はライブで行われるため、同じスケジュールされたインポートを何度も再利用できます。

### 例

[CSVファイルの例はこちら](/csv/fastcomments-scheduled-comments-example.csv)。
