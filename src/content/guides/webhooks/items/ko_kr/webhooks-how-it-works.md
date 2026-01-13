시스템 내 Comment 객체에 대한 모든 변경은 이벤트를 발생시키며 그 이벤트는 큐에 들어갑니다.

초기 웹훅 이벤트는 보통 이벤트 소스가 발생한 후 6초 이내에 전송됩니다.

API가 다운되는 경우를 대비해 Webhooks 관리자에서 이 큐를 모니터링할 수 있습니다.

귀하의 API에 대한 요청이 실패하면, 우리는 그것을 일정에 따라 다시 큐에 넣습니다.

That schedule is `1 Minute * the retry count`. If the call fails once, it'll try again in
a minute. If it fails twice, it'll then wait two minutes, and so on. This is so that we
don't overload your API if you are going down to load related reasons.

웹훅은 [로그 페이지](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs)에서 취소할 수 있습니다.