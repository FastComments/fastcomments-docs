## Example Zaps

몇 분만에 설정할 수 있는 몇 가지 워크플로우.

**새 댓글에 대한 알림 받기.** New Comment, then Slack "Send Channel Message" or Discord "Send  
Channel Message". 댓글 작성자 이름, 댓글 텍스트, 페이지 URL을 메시지에 매핑합니다. 도메인 필터를 추가하여 사이트별로 다른 채널에 알림을 보냅니다.

**모든 댓글을 기록하기.** New Comment, then Google Sheets "Create Spreadsheet Row". 삭제된 댓글을 두 번째 Zap으로 추가하여 댓글 ID가 포함된 행을 추가하면, 시트가 감사 로그 역할도 합니다.

**댓글이 승인될 때 작성자에게 이메일 보내기.** Updated Comment with a Zapier filter on Approved is true, then Gmail "Send Email". Updated Comment는 모든 변경에 대해 발생하므로, 필터가 이 Zap이 승인에만 반응하도록 합니다.

**댓글 작성자를 CRM 또는 메일링 리스트에 추가하기.** New Comment, then HubSpot "Create or Update Contact" or Mailchimp "Add or Update Subscriber" using the commenter email. 마케팅 리스트에 추가하기 전에 개인정보 보호정책 및 현지 법규를 준수하세요.

**폼에서 댓글 만들기.** Typeform or Google Forms "New Response", then FastComments Create Comment with the page URL ID your site uses for testimonials. Approved를 체크하지 않으면 댓글이 표시되기 전에 검토할 수 있습니다.

**공지사항을 피드에 게시하기.** RSS by Zapier "New Item in Feed", then Create Feed Post with the item's title, content, and link.

**회원들을 SSO 사용자로 프로비저닝하기.** Memberstack, Memberful, or your own webhook, then Find SSO User followed by Create SSO User in "find or create" mode.

**신고된 댓글을 에스컬레이션하기.** Updated Comment, filtered on a flag count above zero, then Trello "Create Card" or Linear "Create Issue" with the comment id and a link to the moderation page.

**페이지가 실시간으로 공개될 때 게시하기.** WordPress or Ghost "New Post", then Create Page with the post URL, so the page is listed and restricted before the first comment.

**삭제된 댓글을 보관하기.** Deleted Comment, then Airtable "Create Record" with the full comment for compliance retention.