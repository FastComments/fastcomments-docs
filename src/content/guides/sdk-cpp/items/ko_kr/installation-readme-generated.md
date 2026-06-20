### 종속성 설치

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### 소스에서 빌드하기

```bash
mkdir build
cd build
cmake ..
make
```

### 설치

```bash
sudo make install
```

### 라이브러리 내용

이 라이브러리는 생성된 API 클라이언트와 API 작업을 더 쉽게 해주는 SSO 유틸리티를 포함합니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 공개 vs 보안 API

For the API client, there are three classes, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains
methods that can be made directly from a browser/mobile device/etc without authentication. The `ModerationApi` contains methods that power the moderator dashboard - listing,
counting, searching, exporting and pulling logs for comments, moderation actions (remove/restore, flag, set review/spam/approval status, adjust votes, reopen/close threads),
bans (ban from a comment, undo bans, pre-ban summaries, ban status and preferences, banned-user counts), and badges & trust (award/remove badges, manual badges, get/set trust
factor, user internal profile). Every `ModerationApi` method accepts an `sso` parameter so the call is performed on behalf of an SSO-authenticated moderator.