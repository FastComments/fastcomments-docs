당신이 `Administrators`로 만들고 싶지 않을 수 있는 개발자에게는, 다음 권한을 가진 `Administrator` 사용자를 생성하는 것을 고려하세요:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

이 권한 집합은 개발자가 FastComments를 설정하는 데 필요한 모든 것과 시스템이 제대로 작동하는지 확인하는 데 필요한 가시성을 제공합니다.

이 권한들이 필요한 이유는 다음과 같습니다:

1. **Analytics Admin**: 이는 FastComments 사용 현황 디버깅에 사용할 수 있습니다.
2. **Customizations Admin**: 댓글 위젯에 커스텀 스타일을 적용하려면 필요합니다.
3. **Data Management Admin**: 가져오기 및 내보내기 관리를 하고 웹후크를 설정하려면 필요합니다.
4. **Comment Moderation Admin**: 적어도 설정 중에는 댓글 데이터를 보기 위해 필요합니다.
5. **API/SSO Admin**: 이를 통해 플랫폼에서 API 키를 직접 가져올 수 있습니다. 우리는 이것이 `Administrator`가 이를 복사하여 API Secret을 이메일로 보내는 것보다 더 안전하다고 간주합니다. 이메일 전송은 그리 안전하지 않을 수 있습니다.