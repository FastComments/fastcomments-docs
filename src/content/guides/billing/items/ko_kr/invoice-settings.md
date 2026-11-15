[Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings) 페이지는 Billing Info의 `Invoice Settings` 버튼을 통해 접근할 수 있으며, 인보이스에 표시될 내용과 전송 위치를 제어합니다. `Account Owner` 또는 `Billing Admin`만 이 페이지를 열 수 있습니다.

- `Company Name`: 대시보드, 이메일, 팀 초대에 표시되는 계정 이름입니다. 이것은 청구서 이름이 아닙니다.
- `Billing Email`: 인보이스와 청구 알림이 전송되는 이메일 주소입니다. 계정 소유자가 이를 받지 않아야 한다면 재무 또는 회계 메일함으로 설정하세요. 이 설정은 어떤 사용자의 로그인 이메일도 변경하지 않습니다.
- `Invoice Currency`: 인보이스가 발행되는 통화입니다. [Currencies](#currencies)를 참조하세요.
- `Billing Address`: 인보이스에 인쇄되는 `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code`, `Country`입니다. 카드를 저장할 때 Billing Info 양식에도 동일한 필드가 나타나며, 두 곳 모두 같은 위치에 저장됩니다.
- `VAT / Tax ID`: 선택 사항입니다. 인보이스에 인쇄됩니다.

`Save Changes`를 클릭하세요. 페이지에 `Invoice settings updated successfully.`라는 확인 메시지가 표시되며, 변경 사항은 다음 인보이스부터 적용됩니다. 기존 인보이스는 변경되지 않습니다.

### VAT 및 Tax ID

ID를 입력하기 전에 청구 `Country`를 먼저 선택하세요. 국가에 따라 ID의 종류가 결정되기 때문입니다. EU 회원국은 EU VAT 번호, 영국은 UK VAT 번호, 호주는 ABN, 캐나다는 GST/HST 번호, 미국은 EIN이 사용되며, Stripe이 지원하는 다른 국가들도 마찬가지입니다.

ID는 저장 시점에 유효성이 검사됩니다. 선택한 국가에 대해 유효하지 않은 경우 페이지에 그 사실이 표시되고 ID는 저장되지 않지만, 다른 설정은 저장됩니다. 국가가 세금 ID를 등록할 수 있는 대상이 아닌 경우에도 페이지에서 알려줍니다.

EU 청구 주소를 가지고 있는 경우, Flex 인보이스는 역과세(reverse charge) 거래로 0% VAT로 발행되며, VAT 번호가 저장되면 인보이스에 표시되므로 회계사가 귀하 측의 VAT를 처리할 수 있습니다.