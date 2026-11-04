태그된 사용자는 댓글에서 자신이 태그되었거나 멘션되었다는 사실을 알려주는 이메일을 받게 됩니다.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='@winrid 멘션이 굵게 표시된 댓글을 인용한 알림 이메일 본문, 보기 및 수신 거부 링크 포함'; title='사용자 멘션 알림' app-screenshot-end]

알림을 끄면 이러한 이메일이 발송되지 않으며, 모든 이메일에는 헤더가 포함되어 있어 이메일 클라이언트가 사용자가 원활하게 수신을 거부할 수 있도록 합니다.

태그된 사용자는 이메일 주소가 등록되어 있지 않은 경우에도 인앱 알림을 받습니다.

댓글이 관리자 승인 대기 중인 경우, 멘션 이메일은 보류되었다가 댓글이 승인되면 발송됩니다. 댓글이 스팸으로 표시되거나, 삭제되거나, 7일 이내에 승인되지 않으면 발송되지 않습니다.

멘션된 사용자에게 이메일이 발송되었는지, 또는 발송되지 않은 이유를 확인하려면 댓글 관리 페이지에서 해당 댓글의 [댓글 로그](/guide-moderation.html#comment-logs)를 열어 확인하세요.