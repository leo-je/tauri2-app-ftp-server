export default {
  app: {
    title: 'FTP Server',
    name: 'FTP サーバー'
  },
  splash: {
    welcome: 'FTP Server へようこそ',
    loading: '起動中',
    ready: '準備完了',
    init: {
      title: 'システム初期化',
      systemCheck: 'システムチェック',
      systemCheckDesc: 'OSとハードウェア環境を検出中',
      configLoad: '設定読み込み',
      configLoadDesc: 'アプリケーション設定を読み込み中',
      serviceInit: 'サービス初期化',
      serviceInitDesc: 'FTPサービスとネットワークインターフェースを初期化中',
      ready: '準備完了',
      readyDesc: 'システム初期化が完了しました',
    },
    status: {
      pending: '待機中',
      running: '進行中',
      completed: '完了',
      error: 'エラー',
    },
    error: {
      title: '初期化失敗',
      retry: '再試行',
      skip: 'スキップ',
      unknownError: '不明なエラー',
    },
    messages: {
      checkingSystem: 'システム環境をチェック中...',
      loadingConfig: '設定を読み込み中...',
      initService: 'サービスを初期化中...',
      finalizing: '初期化を完了中...',
      systemCheckComplete: 'チェック完了: {0} {1}',
      configLoaded: '設定を読み込みました',
      usingDefaultConfig: 'デフォルト設定を使用',
      interfacesFound: '{0} 個のネットワークインターフェースを発見',
      systemReady: 'システム準備完了'
    }
  },
  status: {
    running: '実行中',
    stopped: '停止',
    runtime: '実行時間'
  },
  config: {
    title: 'サーバー設定',
    shareDir: '共有ディレクトリ',
    port: 'サービスポート',
    select: '選択',
    open: '開く',
    placeholder: {
      dir: '共有ディレクトリを選択してください',
      port: '21'
    }
  },
  connection: {
    title: '接続アドレス',
    copied: 'クリップボードにコピーしました'
  },
  control: {
    start: 'サービス開始',
    stop: 'サービス停止',
    startTooltip: 'サービス開始',
    stopTooltip: 'サービス停止'
  },
  message: {
    selectPath: 'パスを選択してください',
    noDir: 'ディレクトリが選択されていません',
    pathInvalid: '無効なパス',
    portInvalid: '無効なポート',
    serviceStarted: 'サービスが開始されました',
    serviceStopped: 'サービスが停止しました',
    serviceStartFailed: 'サービスの開始に失敗しました',
    serviceStopFailed: 'サービスの停止に失敗しました'
  },
  validation: {
    portWarning: {
      privilegedPort: '特権ポート {port} の使用には管理者権限が必要になる場合があります',
      ftpStandardPort: '標準 FTP ポート 21 の使用には管理者権限が必要になる場合があります'
    }
  },
  language: {
    title: '言語',
    zh: '中文',
    en: 'English',
    ja: '日本語',
    th: 'ไทย'
  },
  tabs: {
    server: 'サーバー',
    auth: '設定',
    about: 'について'
  },
  auth: {
    serverRunning: 'サービス実行中、権限設定はロックされています',
    settings: '一般設定',
    autoStart: '自動起動',
    autoStartDesc: 'システム起動時にアプリケーションを自動起動',
    hideOnStartup: '起動時にメインウィンドウを非表示',
    hideOnStartupDesc: '起動時にメインウィンドウを非表示にし、トレイアイコンのみ表示',
    anonymous: '匿名アクセス',
    anonymousDesc: '認証なしでアクセス可能',
    readOnly: '読み取り専用',
    readWrite: '読み書き',
    userList: 'ユーザーリスト',
    addUser: 'ユーザーを追加',
    username: 'ユーザー名',
    password: 'パスワード',
    actions: '操作',
    edit: '編集',
    delete: '削除',
    noUsers: 'ユーザーがいません',
    addUserHint: '上の「ユーザーを追加」ボタンをクリックして新規ユーザーを作成',
    editUser: 'ユーザーを編集',
    usernamePlaceholder: 'ユーザー名を入力',
    passwordPlaceholder: 'パスワードを入力',
    permission: '権限',
    cancel: 'キャンセル',
    save: '保存',
    deleteConfirm: 'ユーザー「{username}」を削除してもよろしいですか？',
    deleteTitle: '削除確認',
    confirm: '確認',
    userDeleted: 'ユーザーを削除しました',
    userUpdated: 'ユーザーを更新しました',
    userAdded: 'ユーザーを追加しました',
    saveFailed: 'データの保存に失敗しました'
  },
  about: {
    appName: 'FTP サーバー',
    description: 'Tauri 2.0 で構築された軽量でクロスプラットフォームな FTP サーバー管理ツール。モダンなユーザーインターフェースを備え、迅速なデプロイと安全なファイル転送サービスを提供します。',
    techStack: '技術スタック',
    features: '主な機能',
    featureList: {
      lightweight: '軽量設計でメモリ使用量が少ない',
      crossPlatform: 'Windows、macOS、Linux に対応',
      secure: 'ユーザー権限管理で安全・信頼性が高い',
      easyToUse: 'シンプルで直感的なユーザーインターフェース',
      modern: 'モダンな技術スタックとデザイン哲学'
    },
    links: {
      github: 'GitHub リポジトリ',
      documentation: 'ドキュメント'
    },
    copyright: '© 2024 FTP Server. All rights reserved.',
    rights: '全著作権所有',
    license: 'Apache 2.0 ライセンスでオープンソース'
  },
  tray: {
    show: 'メインウィンドウを表示',
    start: 'FTP を起動',
    stop: 'FTP を停止',
    quit: '終了',
    uptime: '稼働時間'
  }
}
