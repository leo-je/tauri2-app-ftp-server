export default {
  app: {
    title: 'FTP Server',
    name: 'FTP Server'
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
    en: 'English'
  },
  tabs: {
    server: 'Server',
    auth: 'Permissions'
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
  }
}
