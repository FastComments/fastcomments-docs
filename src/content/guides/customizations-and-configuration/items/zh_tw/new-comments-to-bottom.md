[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

預設情況下，新即時評論會在發佈時即時顯示於評論列表的最上方。

啟用此選項後，新即時評論會改為加入列表底部。這會影響使用者在瀏覽評論串時，即時發佈的評論顯示方式。

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

啟用此設定後：
- 其他使用者發表的新即時評論會顯示在評論列表的底部
- 使用者會即時看到新評論出現在現有評論的下方
- 這只會影響即時評論更新 - 不會影響初始頁面載入時的排序
- 當使用者在追蹤討論時，這有助於維持閱讀流程

請注意，此設定僅影響新即時評論在即時到達時的放置位置。它不會影響頁面載入時的初始排序。