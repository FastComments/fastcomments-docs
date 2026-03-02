#### 플러그인 다운로드

최신 릴리스 ZIP 파일을 <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub 리포지토리</a>에서 다운로드하세요.

#### Moodle 디렉터리에 압축 풀기

플러그인이 `<moodle-root>/local/fastcomments` 위치에 설치되도록 ZIP 파일을 Moodle 설치 디렉터리에 압축 해제하세요. 플러그인 디렉터리에는 `version.php`, `lib.php` 및 기타 플러그인 파일들이 직접 있어야 하며(하위 폴더에 중첩되어 있으면 안 됩니다).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Moodle 관리자에서 설치

사이트 관리자 계정으로 로그인한 후 **Site Administration > Notifications**로 이동하세요. Moodle이 새 플러그인을 감지하고 설치 실행을 요청합니다.

#### 플러그인 구성

설치 후 **Site Administration > Plugins > Local plugins > FastComments**로 이동하여 설정을 입력하세요. 각 옵션에 대한 자세한 내용은 [구성](#items-moodle-configuration) 섹션을 참조하세요.