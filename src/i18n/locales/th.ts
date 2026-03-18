export default {
  app: {
    title: 'FTP Server',
    name: 'FTP เซิร์ฟเวอร์'
  },
  splash: {
    welcome: 'ยินดีต้อนรับสู่ FTP Server',
    loading: 'กำลังเริ่มต้น',
    ready: 'พร้อมใช้งาน',
    init: {
      title: 'เริ่มต้นระบบ',
      systemCheck: 'ตรวจสอบระบบ',
      systemCheckDesc: 'ตรวจหา OS และสภาพแวดล้อมฮาร์ดแวร์',
      configLoad: 'โหลดการตั้งค่า',
      configLoadDesc: 'กำลังโหลดการตั้งค่าแอปพลิเคชัน',
      serviceInit: 'เริ่มต้นบริการ',
      serviceInitDesc: 'กำลังเริ่มต้นบริการ FTP และอินเทอร์เฟซเครือข่าย',
      ready: 'พร้อมใช้งาน',
      readyDesc: 'การเริ่มต้นระบบเสร็จสมบูรณ์',
    },
    status: {
      pending: 'รอดำเนินการ',
      running: 'กำลังดำเนินการ',
      completed: 'เสร็จสมบูรณ์',
      error: 'ข้อผิดพลาด',
    },
    error: {
      title: 'การเริ่มต้นล้มเหลว',
      retry: 'ลองใหม่',
      skip: 'ข้าม',
      unknownError: 'ข้อผิดพลาดที่ไม่รู้จัก',
    },
    messages: {
      checkingSystem: 'กำลังตรวจสอบสภาพแวดล้อมระบบ...',
      loadingConfig: 'กำลังโหลดการตั้งค่า...',
      initService: 'กำลังเริ่มต้นบริการ...',
      finalizing: 'กำลังเสร็จสิ้นการเริ่มต้น...',
      systemCheckComplete: 'ตรวจสอบเสร็จสมบูรณ์: {0} {1}',
      configLoaded: 'โหลดการตั้งค่าเสร็จสมบูรณ์',
      usingDefaultConfig: 'ใช้การตั้งค่าเริ่มต้น',
      interfacesFound: 'พบ {0} อินเทอร์เฟซเครือข่าย',
      systemReady: 'ระบบพร้อมใช้งาน'
    }
  },
  status: {
    running: 'กำลังทำงาน',
    stopped: 'หยุดทำงาน',
    runtime: 'เวลาทำงาน'
  },
  config: {
    title: 'การตั้งค่าเซิร์ฟเวอร์',
    shareDir: 'ไดเรกทอรีที่แชร์',
    port: 'พอร์ตบริการ',
    select: 'เลือก',
    open: 'เปิด',
    placeholder: {
      dir: 'กรุณาเลือกไดเรกทอรีที่แชร์',
      port: '21'
    }
  },
  connection: {
    title: 'ที่อยู่การเชื่อมต่อ',
    copied: 'คัดลอกไปยังคลิปบอร์ดแล้ว'
  },
  control: {
    start: 'เริ่มบริการ',
    stop: 'หยุดบริการ',
    startTooltip: 'เริ่มบริการ',
    stopTooltip: 'หยุดบริการ'
  },
  message: {
    selectPath: 'กรุณาเลือกเส้นทาง',
    noDir: 'ยังไม่ได้เลือกไดเรกทอรี',
    pathInvalid: 'เส้นทางไม่ถูกต้อง',
    portInvalid: 'พอร์ตไม่ถูกต้อง',
    serviceStarted: 'บริการเริ่มแล้ว',
    serviceStopped: 'บริการหยุดแล้ว',
    serviceStartFailed: 'การเริ่มต้นบริการล้มเหลว',
    serviceStopFailed: 'การหยุดบริการล้มเหลว'
  },
  validation: {
    portWarning: {
      privilegedPort: 'การใช้พอร์ตพิเศษ {port} อาจต้องการสิทธิ์ผู้ดูแลระบบ',
      ftpStandardPort: 'การใช้พอร์ต FTP มาตรฐาน 21 อาจต้องการสิทธิ์ผู้ดูแลระบบ'
    }
  },
  language: {
    title: 'ภาษา',
    zh: '中文',
    en: 'English',
    ja: '日本語',
    th: 'ไทย'
  },
  tabs: {
    server: 'เซิร์ฟเวอร์',
    auth: 'การตั้งค่า',
    about: 'เกี่ยวกับ'
  },
  auth: {
    serverRunning: 'บริการกำลังทำงาน การตั้งค่าสิทธิ์ถูกล็อก',
    settings: 'การตั้งค่าทั่วไป',
    autoStart: 'เริ่มอัตโนมัติ',
    autoStartDesc: 'เปิดแอปพลิเคชันโดยอัตโนมัติเมื่อเริ่มระบบ',
    hideOnStartup: 'ซ่อนหน้าต่างหลักเมื่อเริ่มต้น',
    hideOnStartupDesc: 'ซ่อนหน้าต่างหลักเมื่อเริ่มต้น แสดงเฉพาะไอคอนในถาด',
    anonymous: 'การเข้าถึงแบบไม่ระบุตัวตน',
    anonymousDesc: 'เข้าถึงได้โดยไม่ต้องยืนยันตัวตน',
    readOnly: 'อ่านอย่างเดียว',
    readWrite: 'อ่านและเขียน',
    userList: 'รายชื่อผู้ใช้',
    addUser: 'เพิ่มผู้ใช้',
    username: 'ชื่อผู้ใช้',
    password: 'รหัสผ่าน',
    actions: 'การดำเนินการ',
    edit: 'แก้ไข',
    delete: 'ลบ',
    noUsers: 'ยังไม่มีผู้ใช้',
    addUserHint: 'คลิกปุ่ม "เพิ่มผู้ใช้" ด้านบนเพื่อสร้างผู้ใช้ใหม่',
    editUser: 'แก้ไขผู้ใช้',
    usernamePlaceholder: 'กรุณากรอกชื่อผู้ใช้',
    passwordPlaceholder: 'กรุณากรอกรหัสผ่าน',
    permission: 'สิทธิ์',
    cancel: 'ยกเลิก',
    save: 'บันทึก',
    deleteConfirm: 'คุณแน่ใจหรือไม่ว่าต้องการลบผู้ใช้ "{username}"?',
    deleteTitle: 'ยืนยันการลบ',
    confirm: 'ยืนยัน',
    userDeleted: 'ลบผู้ใช้แล้ว',
    userUpdated: 'อัปเดตผู้ใช้แล้ว',
    userAdded: 'เพิ่มผู้ใช้แล้ว',
    saveFailed: 'บันทึกข้อมูลล้มเหลว'
  },
  about: {
    appName: 'FTP เซิร์ฟเวอร์',
    version: 'เวอร์ชัน',
    clickToCopy: 'คลิกเพื่อคัดลอกหมายเลขเวอร์ชัน',
    versionCopied: 'คัดลอกหมายเลขเวอร์ชันแล้ว',
    developer: 'ผู้พัฒนา',
    developerName: 'FTP Server Team',
    description: 'เครื่องมือจัดการเซิร์ฟเวอร์ FTP ข้ามแพลตฟอร์มที่เบาและสร้างด้วย Tauri 2.0 มีอินเทอร์เฟซผู้ใช้ที่ทันสมัยสำหรับการปรับใช้ที่รวดเร็วและบริการถ่ายโอนไฟล์ที่ปลอดภัย',
    techStack: 'เทคโนโลยีที่ใช้',
    features: 'คุณสมบัติหลัก',
    featuresList: {
      crossPlatform: {
        title: 'รองรับหลายแพลตฟอร์ม',
        desc: 'รองรับ Windows, macOS และ Linux'
      },
      easyConfig: {
        title: 'ตั้งค่าง่าย',
        desc: 'อินเทอร์เฟซที่ใช้งานง่าย เริ่มต้นบริการ FTP ได้อย่างรวดเร็ว'
      },
      permissionControl: {
        title: 'จัดการสิทธิ์',
        desc: 'การควบคุมสิทธิ์ผู้ใช้และการเข้าถึงแบบไม่ระบุตัวตนที่ยืดหยุ่น'
      },
      realtime: {
        title: 'ตรวจสอบแบบเรียลไทม์',
        desc: 'ดูสถานะบริการและเวลาทำงาน'
      }
    },
    linksSection: 'ลิงก์ที่เกี่ยวข้อง',
    links: {
      github: 'GitHub',
      documentation: 'เอกสารประกอบ',
      feedback: 'แจ้งปัญหา',
      changelog: 'บันทึกการเปลี่ยนแปลง'
    },
    copyright: 'ลิขสิทธิ์ © 2024 FTP Server Team',
    rights: 'สงวนลิขสิทธิ์ทั้งหมด'
  },
  tray: {
    show: 'แสดงหน้าต่างหลัก',
    start: 'เริ่ม FTP',
    stop: 'หยุด FTP',
    quit: 'ออก',
    uptime: 'เวลาทำงาน'
  }
}
