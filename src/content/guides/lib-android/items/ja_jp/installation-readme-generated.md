アプリの `build.gradle.kts` ファイルに FastComments SDK を追加してください:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

プロジェクトの `settings.gradle.kts` に Repsy リポジトリが含まれていることを確認してください:

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