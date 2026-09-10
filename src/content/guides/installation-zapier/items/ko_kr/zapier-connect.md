## 계정 연결

1. Zapier에서 FastComments 단계를 Zap에 추가하거나 Zapier 앱 디렉터리에서 FastComments 앱 페이지를 엽니다.  
2. **Sign in to FastComments**를 선택합니다. Zapier는 먼저 지역을 묻습니다: 계정이 EU 지역(`eu.fastcomments.com`)에서 생성되지 않은 경우 **United States**를 선택하세요.  
3. FastComments 창이 열립니다. 아직 로그인하지 않았다면 로그인합니다.  
4. 동의 페이지를 검토합니다. 여기에는 Zapier 애플리케이션, 연결될 계정, 요청된 권한(읽기 및 쓰기)이 표시됩니다. **Approve**를 선택합니다.  
5. Zapier는 연결을 저장하고 사이트 이름 및 사용자 이름으로 라벨을 지정합니다.  

연결은 OAuth를 사용합니다. API 키가 Zapier에 복사되지 않으며, Zapier가 보유한 토큰은 승인한 계정에만 작동합니다.

## 연결할 수 있는 사람

연결을 승인하는 사람은 FastComments 계정의 **API admin**이어야 합니다. 계정 소유자는 이 권한을 가지고 있으며, 다른 팀 구성원은 Users 페이지에서 부여받을 수 있습니다. 이 권한이 없는 사용자는 "you do not have permission" 페이지를 보게 됩니다.

## 올바른 사이트 연결

동의 페이지는 현재 로그인한 계정에 연결합니다. 여러 계정을 관리하는 경우 승인하기 전에 계정 전환기에서 올바른 계정으로 전환하거나 동의 페이지의 **switch account** 링크를 사용하세요. Zapier의 연결 라벨에 사이트 이름이 표시되므로 잘못된 선택을 쉽게 확인할 수 있습니다.

## 접근 검토 및 해제

모든 연결은 FastComments 대시보드의 **Connected Apps**에 표시되며, 보유한 권한 및 마지막 사용 시점이 표시됩니다. 여기서 연결을 해제하면 Zapier가 즉시 연결을 끊으며, 해당 연결을 사용하는 모든 Zap은 재연결될 때까지 중단됩니다. Zapier 측에서는 **My Apps**에서 연결을 제거할 수도 있습니다.