기본적으로 FastComments는 하루에 한 번 WordPress 사이트로 동기화됩니다. 이는 데이터의 사본을 계속 보유하기 위한 백업 목적과 일부 플러그인이 이 데이터를 필요로 할 수 있기 때문입니다.

일부 사이트는 많은 읽기 트래픽을 처리할 수 있지만 데이터베이스 구성은 대량의 쓰기 트래픽을 항상 감당하지 못하기 때문에, 모든 댓글이 저장될 때마다 즉시 동기화가 이루어지지는 않습니다(따라서 이 작업을 FastComments에 오프로드합니다).

WordPress로의 동기화 일정은 플러그인을 설치하여 사용자 지정할 수 있습니다. [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description)을 권장합니다.

Steps:

1. WP Crontrol을 설치합니다
2. `Settings -> Cron Schedules`로 이동합니다.
3. `Cron Events` 탭으로 이동합니다.
4. `fastcomments_cron_hook`을 검색합니다.
5. 이벤트를 편집합니다. 훅을 시간 단위로, 하루 두 번, 일간(기본) 또는 주간으로 실행되도록 구성할 수 있습니다.

WordPress로의 동기화는 FastComments 플러그인 대시보드로 이동하여 `Manually Sync`를 선택하면 언제든지 수동으로 수행할 수도 있습니다. 옵션을 선택할 수 있습니다
WP 설치로 동기화하거나 WP 댓글을 FastComments 서버에 다시 업로드할 수 있습니다.