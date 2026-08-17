---
アプリの `build.gradle.kts` ファイルに FastComments SDK を追加してください:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

プロジェクトの `settings.gradle.kts` に Repsy リポジトリが設定されていることを確認してください:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```
---