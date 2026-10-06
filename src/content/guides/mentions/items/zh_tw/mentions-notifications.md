---
被標記的使用者將收到一封電子郵件，告知他們在評論中被標記或提及。

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='通知電子郵件內容引用帶有 @winrid 提及的粗體評論，並附有檢視與取消訂閱連結'; title='使用者提及通知' app-screenshot-end]

關閉通知將阻止這些電子郵件，且每封郵件都會包含標頭，使電子郵件客戶端能順利讓使用者取消訂閱。

即使沒有註冊電子郵件地址，被標記的使用者仍會收到應用內通知。

如果評論正等待審核，提及郵件會被暫存，待評論獲批准後再發送。若評論被標記為垃圾郵件、已刪除，或在 7 天內未獲批准，則不會發送。

若要查看被提及的使用者是否收到郵件，或了解未收到的原因，請從「審核評論」頁面開啟該評論的[評論日誌](/guide-moderation.html#comment-logs)。

---