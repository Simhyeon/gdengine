### Gdengine, 게임 디자인 엔진

본 프로젝트는 게임 디자인 툴 [gdmarp](https://github.com/Simhyeon/gdmarp/)의
새로운 크로스 플랫폼 프론트엔드임

### 설치

[배포 페이지](https://github.com/Simhyeon/gdengine/releases/new)를 참고

### Gdengine은 무엇을 하는가

Gdengine은 게임 디자인 문서 작성 툴임. Gdengine은 2가지 작업을 실행함. 첫번째로
인덱스 파일에 있는 매크로를 처리함. 두번째는 인덱스 파일을 타겟 포맷으로
렌더함.

Gdengine에는 다양한 매크로들이 손 쉬운 문서 작성을 위하여 포함되어 있음. 또한
매크로는 인덱스 파일을 수정하지 않고도 여러가지 포맷으로 렌더할 수 있도록
도움을 줌.

### 의존성 

- --git 옵션을 위한 **Git** 바이너리

- flowchartgvz(정적 플로우차트) : [graphviz](https://graphviz.org/)
- gdlogue(다이얼로그,회화) : [graphviz](https://graphviz.org/)

### 렌더러

**문서 렌더러**
- marp (슬라이드 파일 html,pdf,pptx)
- mediawiki (웹위키 페이지)
- pandoc (docx)

**컴포넌트 렌더러**
- flowchartjs (플로우차트 html)
- flowchartgvz (플로우차트 png, pdf)
- gdlogue (다이얼로그, html, png, pdf)
- webuibts (웹 기반 UI)

### 사용법

```bash
# 프로젝트 초기화
gde init --git

# Marp 백엔드로 렌더링
gde render -m marp
```

### 사용례(영문)

[Usage](docs/usage.md)

### 매크로(영문)

[Macros](docs/macro.md)
