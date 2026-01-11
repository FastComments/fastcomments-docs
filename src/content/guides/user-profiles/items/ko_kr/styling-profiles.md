사용자 프로필이 사이트(댓글 위젯을 통해)의 컨텍스트에서 열릴 때, FastComments 위젯에 적용한 모든 사용자 정의 CSS 스타일이 프로필 모달에 자동으로 주입됩니다.

### 작동 방식

사용자가 댓글 위젯에서 프로필 링크를 클릭하면 클래스 `.fast-comments-profile`를 가진 프로필 모달이 열립니다. 위젯의 사용자 정의 CSS는 프로필 뷰에 자동으로 주입됩니다. 이미 댓글 위젯을 스타일링한 경우, 해당 스타일이 프로필에도 적용됩니다.

### CSS 클래스

FastComments 프로필은 클래스 기반 CSS 아키텍처를 사용합니다. CSS 커스텀 프로퍼티는 사용하지 않습니다.

메인 프로필 페이지는 루트 컨테이너로 `.user-profile`를 사용합니다. 헤더 섹션은 `.profile-header`이며 배경 이미지는 `.profile-header-background`입니다. 프로필 콘텐츠는 `.profile-content`에 위치합니다.

아바타는 `.profile-avatar`와 `.profile-avatar-wrapper`를 사용합니다. 사용자의 이름은 `.profile-name`이고 소개 텍스트는 `.profile-bio`입니다. 통계는 `.profile-stats`에 있으며 개별 통계는 `.stat`을 사용합니다.

소셜 링크는 `.profile-social-links`에 있고 개별 링크는 `.social-link`입니다. 배지는 `.profile-badges`와 `.badge`를 사용합니다. 배지 진행바는 `.progress-outer`와 `.progress-bar`를 사용합니다.

탭은 컨테이너에 `.profile-tabs`를 사용하고 개별 탭에는 `.tab`, 선택된 탭에는 `.tab.active`를 사용합니다. 탭 내용은 `.tab-body` 및 `.tab-body.active`를 사용합니다. 탭의 알림 수는 `.tab .count`를 사용합니다.

알림은 `.notification`을 사용하고 DM 대화는 `.conversation`을 사용합니다. 온라인 상태는 `.activity-indicator`이며 활성 상태는 `.activity-indicator.online`입니다. 읽지 않은 카운터는 `.unread-count`를 사용합니다.

프로필 모달 컨테이너는 `.fast-comments-profile`이며 닫기 버튼은 `.fast-comments-profile-close`입니다.

### 다크 모드

다크 모드는 `.user-profile`에 대한 `.dark` 클래스 수정자를 사용합니다.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### 예제

**헤더:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**배지:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**탭:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**모달:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```