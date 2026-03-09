export default {
  app: {
    title: 'FTP Server',
    name: 'FTP Server'
  },
  splash: {
    welcome: 'Welcome to FTP Server',
    loading: 'Loading',
    ready: 'Ready',
    init: {
      title: 'System Initialization',
      systemCheck: 'System Check',
      systemCheckDesc: 'Detecting OS and hardware environment',
      configLoad: 'Configuration',
      configLoadDesc: 'Loading application configuration',
      serviceInit: 'Service Initialization',
      serviceInitDesc: 'Initializing FTP service and network interfaces',
      ready: 'Ready',
      readyDesc: 'System initialization completed',
    },
    status: {
      pending: 'Pending',
      running: 'In Progress',
      completed: 'Completed',
      error: 'Error',
    },
    error: {
      title: 'Initialization Failed',
      retry: 'Retry',
      skip: 'Skip',
      unknownError: 'Unknown Error',
    },
    messages: {
      checkingSystem: 'Checking system environment...',
      loadingConfig: 'Loading configuration...',
      initService: 'Initializing services...',
      finalizing: 'Finalizing initialization...',
      systemCheckComplete: 'Check complete: {0} {1}',
      configLoaded: 'Config loaded',
      usingDefaultConfig: 'Using default config',
      interfacesFound: 'Found {0} network interfaces',
      systemReady: 'System ready'
    }
  },
  status: {
    running: 'Running',
    stopped: 'Stopped',
    runtime: 'Runtime'
  },
  config: {
    title: 'Server Configuration',
    shareDir: 'Shared Directory',
    port: 'Service Port',
    select: 'Select',
    open: 'Open',
    placeholder: {
      dir: 'Please select shared directory',
      port: '21'
    }
  },
  connection: {
    title: 'Connection Address',
    copied: 'Copied to clipboard'
  },
  control: {
    start: 'Start Service',
    stop: 'Stop Service',
    startTooltip: 'Start Service',
    stopTooltip: 'Stop Service'
  },
  message: {
    selectPath: 'Please select a path',
    noDir: 'No directory selected',
    pathInvalid: 'Invalid path',
    portInvalid: 'Invalid port'
  },
  language: {
    title: 'Language',
    zh: '中文',
    en: 'English',
    ja: '日本語',
    th: 'ไทย'
  },
  tabs: {
    server: 'Server',
    auth: 'Permissions',
    about: 'About'
  },
  auth: {
    serverRunning: 'Server is running, settings are locked',
    anonymous: 'Anonymous Access',
    anonymousDesc: 'Access without authentication',
    readOnly: 'Read Only',
    readWrite: 'Read & Write',
    userList: 'User List',
    addUser: 'Add User',
    username: 'Username',
    password: 'Password',
    actions: 'Actions',
    edit: 'Edit',
    delete: 'Delete',
    noUsers: 'No users yet',
    addUserHint: 'Click "Add User" button above to create a new user',
    editUser: 'Edit User',
    usernamePlaceholder: 'Enter username',
    passwordPlaceholder: 'Enter password',
    permission: 'Permission',
    cancel: 'Cancel',
    save: 'Save',
    deleteConfirm: 'Are you sure you want to delete user "{username}"?',
    deleteTitle: 'Confirm Delete',
    confirm: 'Confirm',
    userDeleted: 'User deleted',
    userUpdated: 'User updated',
    userAdded: 'User added',
    saveFailed: 'Failed to save data'
  },
  about: {
    appName: 'FTP Server',
    version: 'Version',
    clickToCopy: 'Click to copy version',
    versionCopied: 'Version copied',
    developer: 'Developer',
    developerName: 'FTP Server Team',
    description: 'A lightweight, cross-platform FTP server management tool built with Tauri 2.0. Featuring a modern user interface for quick deployment and secure file transfer services.',
    techStack: 'Tech Stack',
    features: 'Key Features',
    featuresList: {
      crossPlatform: {
        title: 'Cross-Platform',
        desc: 'Supports Windows, macOS, and Linux'
      },
      easyConfig: {
        title: 'Easy Configuration',
        desc: 'Intuitive interface for quick FTP setup'
      },
      permissionControl: {
        title: 'Permission Control',
        desc: 'Flexible user permissions and anonymous access'
      },
      realtime: {
        title: 'Real-time Monitoring',
        desc: 'Track service status and runtime'
      }
    },
    linksSection: 'Links',
    links: {
      github: 'GitHub',
      documentation: 'Documentation',
      feedback: 'Feedback',
      changelog: 'Changelog'
    },
    copyright: 'Copyright © 2024 FTP Server Team',
    rights: 'All rights reserved'
  }
}
