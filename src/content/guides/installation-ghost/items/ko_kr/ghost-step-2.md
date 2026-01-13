이제 zip 파일을 다운로드했으므로 폴더에 압축을 푸세요. 기본 `casper.zip`을 다운로드하여 Windows의 Downloads\casper에 추출했습니다.

다음으로 LTS 버전 이상의 NodeJS가 설치되어 있는지 확인하세요. 다음에서 다운로드할 수 있습니다: https://nodejs.org/en/download/

NodeJS가 설치되면 코드 편집기를 설치하세요.

추천(및 당사에서 사용하는) 편집기는 WebStorm이며, 30일 체험판(신용카드 불필요)으로 여기에서 받을 수 있습니다: https://www.jetbrains.com/webstorm/

다음으로 가장 좋은 무료 옵션은 아마 Visual Studio Code일 것입니다: https://code.visualstudio.com/download

편집기를 설정하고 에디터에서 테마 폴더를 연 후, IDE에서 터미널을 열고 다음을 실행하세요:

[inline-code-attrs-start title = '테마 설치'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

성공적인 출력은 다음과 같습니다(경고는 무시하셔도 됩니다):

<div class="screenshot white-bg">
    <div class="title">npm 설치 성공 출력</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="npm 설치 성공 출력" />
</div>

이렇게 하면 이후 실행할 명령을 위해 테마의 의존성이 설정됩니다. 또한, the export는 테마의 의존성이 설치되어 있어야 하므로, 그렇지 않으면 the re-import가 제대로 작동하지 않습니다.