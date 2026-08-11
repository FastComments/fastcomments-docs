FastComments는 중재자와 관리자를 위해 일일, 주간 또는 월간 이메일 다이제스트를 지원합니다.

다이제스트 빈도는 <a href="" target="_blank">여기</a>에서 설정할 수 있습니다.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='다이제스트가 일일, 주간 또는 월간으로 도착하도록 설정된 편집 알림 양식'; title='다이제스트 빈도 구성' app-screenshot-end]

전체 댓글 통계를 포함하면서, 검토가 필요한 가장 최근의 세 댓글도 나열됩니다.

각 댓글에 대해 직접 매직 링크가 제공됩니다:
- 댓글 승인.
- 댓글을 검토된 것으로 표시하고 응답 페이지로 이동.
- 댓글을 스팸으로 표시.

각 댓글에 대한 이러한 링크는 자동으로 인증을 수행하고 이메일에서 해당 작업을 수행합니다.

또한, 다이제스트에 있는 '댓글 관리' 버튼은 동일한 인증을 수행하고 댓글 관리 페이지로 이동합니다.

이 매직 링크는 일정 시간이 지나면 만료된다는 점에 유의하십시오.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='댓글 통계와 검토가 필요한 세 댓글이 포함된 월간 다이제스트 이메일이며, 각각 승인, 응답 및 스팸 링크가 있습니다.'; title='다이제스트 이메일' app-screenshot-end]

#### 알림 유형

FastComments는 중재자와 관리자를 위해 다양한 유형의 이메일을 보냅니다. 원한다면 위에 표시된 `Edit Notifications` 페이지에서 적절한 옵션을 선택하여 `Comment Reply` 알림을 수신 거부하고, `New Comment` 알림은 계속 받을 수 있습니다.

---