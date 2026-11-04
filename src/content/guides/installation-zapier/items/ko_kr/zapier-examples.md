## 예제 Zap

몇 분 만에 설정할 수 있는 워크플로우 몇 가지입니다.

**새 댓글에 대한 알림 받기.** New Comment 트리거를 사용한 뒤, Slack의 "Send Channel Message" 또는 Discord의 "Send Channel Message"를 연결하세요. 댓글 작성자 이름, 댓글 내용, 페이지 URL을 메시지에 매핑합니다. 도메인 필터를 추가하면 사이트별로 다른 채널에 알림을 보낼 수 있습니다.

**모든 댓글 기록 유지.** New Comment 트리거를 사용한 뒤, Google Sheets의 "Create Spreadsheet Row"를 연결하세요. Deleted Comment 트리거를 두 번째 Zap으로 추가하여 댓글 id와 함께 행을 추가하면, 스프레드시트가 감사 기록 역할도 겸하게 됩니다.

**댓글이 승인되면 작성자에게 이메일 보내기.** Approved가 true일 때만 통과하도록 Zapier 필터가 설정된 Updated Comment 트리거를 사용한 뒤, Gmail의 "Send Email"을 연결하세요. Updated Comment는 모든 변경 시에 발생하므로, 이 필터가 승인에만 반응하도록 만드는 핵심입니다.

**댓글 작성자를 CRM 또는 메일링 리스트에 추가.** New Comment 트리거를 사용한 뒤, 댓글 작성자의 이메일을 사용하여 HubSpot의 "Create or Update Contact" 또는 Mailchimp의 "Add or Update Subscriber"를 연결하세요. 마케팅 리스트에 누군가를 추가하기 전에 개인정보 처리방침과 현지 법률을 준수해야 합니다.

**폼에서 댓글 생성.** Typeform 또는 Google Forms의 "New Response"를 트리거로 사용한 뒤, 사이트에서 추천사(testimonial)용으로 사용하는 페이지 URL ID로 FastComments의 Create Comment를 연결하세요. Approved를 체크하지 않은 상태로 두면 각 댓글이 표시되기 전에 검토할 수 있습니다.

**피드에 공지 게시.** RSS by Zapier의 "New Item in Feed"를 트리거로 사용한 뒤, 항목의 제목, 내용, 링크와 함께 Create Feed Post를 연결하세요.

**멤버를 SSO 사용자로 프로비저닝.** Memberstack, Memberful 또는 직접 만든 웹훅을 트리거로 사용한 뒤, Find SSO User를 실행하고 이어서 "find or create" 모드로 Create SSO User를 연결하세요.

**신고된 댓글 에스컬레이션.** 플래그 수가 0보다 큰 경우에만 통과하도록 필터링된 Updated Comment 트리거를 사용한 뒤, 댓글 id와 검토 페이지 링크와 함께 Trello의 "Create Card" 또는 Linear의 "Create Issue"를 연결하세요.

**페이지가 라이브되는 대로 게시.** WordPress 또는 Ghost의 "New Post"를 트리거로 사용한 뒤, 게시물 URL과 함께 Create Page를 연결하세요. 그러면 첫 댓글이 달리기 전에 페이지가 목록에 등록되고 접근이 제한됩니다.

**삭제된 댓글 보관.** Deleted Comment 트리거를 사용한 뒤, 컴플라이언스 보존을 위해 전체 댓글 내용과 함께 Airtable의 "Create Record"를 연결하세요.