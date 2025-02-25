<template>
    <div style="text-align:left">
        <ElForm>
            <el-form-item label="是否匿名访问">
                <el-switch v-model="isAnonymous" @change="() => store.set('isAnonymous', isAnonymous)" />
            </el-form-item>
            <el-form-item v-if="isAnonymous" label="权限" label-width="80px"
                @change="() => store.set('fileauth', fileauth)"
            >
                <el-radio-group v-model="fileauth" size="large" >
                    <el-radio-button label="只读" value="R" />
                    <el-radio-button label="读写" value="W" />
                </el-radio-group>
            </el-form-item>
        </ElForm>
        <div v-if="!isAnonymous">
            <ElTable :data="tableData" height="140" border style="width: 100%;">
                <ElTableColumn label="用户名" prop="username" />
                <ElTableColumn label="密码" prop="password" />
                <ElTableColumn label="权限" prop="fileauth">
                    <template #default="scope">
                        <div style="display: flex; align-items: center">
                            <span style="margin-left: 10px">{{ scope.row.fileauth == 'W' ? '读写' : '只读' }}</span>
                        </div>
                    </template>
                </ElTableColumn>
                <el-table-column fixed="right" label="操作" min-width="120">
                    <template #default="scope">
                        <el-button link type="primary" size="small" @click="_e => {
                            console.log(scope)
                            form.index = scope.$index
                            form.username = scope.row.username
                            form.password = scope.row.password
                            form.fileauth = scope.row.fileauth
                            drawer = !drawer
                        }">修改</el-button>
                        <el-button link type="primary" size="small" @click="deleteRow(scope)">删除</el-button>
                    </template>
                </el-table-column>
            </ElTable>
            <div style="text-align: right;">
                <ElButton type="success" @click="() => {
                    drawer = true;
                    form.password = ''
                    form.username = ''
                    form.index = null
                    form.fileauth = 'R'
                }" style="width: 100px;margin-top: 5px;">添加
                </ElButton>
            </div>
        </div>
        <div>
            <el-drawer v-model="drawer" title="I am the title" :with-header="false" :direction="direction" size="90%">
                <el-form style="margin-top: 30px;" :model="form">
                    <el-form-item label="用户名" label-width="80px">
                        <el-input autocomplete="off" v-model="form.username" />
                    </el-form-item>
                    <el-form-item label="密码" label-width="80px">
                        <el-input autocomplete="off" v-model="form.password" />
                    </el-form-item>
                    <el-form-item label="权限" label-width="80px">
                        <el-radio-group v-model="form.fileauth" size="large">
                            <el-radio-button label="只读" value="R" />
                            <el-radio-button label="读写" value="W" />
                        </el-radio-group>
                    </el-form-item>
                </el-form>
                <div style="text-align: right;">
                    <ElButton type="" @click="() => drawer = !drawer">取消</ElButton>
                    <ElButton type="success" @click="saveForm">保存</ElButton>
                </div>
            </el-drawer>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { onMounted, reactive, ref } from 'vue'
import {
    ElSwitch, ElForm, ElFormItem, ElTable, ElTableColumn,
    ElButton, ElDrawer, DrawerProps, ElInput, ElMessage,
    ElRadioGroup, ElRadioButton
} from 'element-plus'
import { Store } from '@tauri-apps/plugin-store';
let store: Store;//await Store.load('store.json');
const isAnonymous = ref(true)
const tableData = ref([{}])
tableData.value = []
const fileauth = ref('R')
const init = async () => {
    if (!store) {
        store = await Store.load('store.json');
    }
    let a = await store.get('isAnonymous');
    isAnonymous.value = a ? true : false

    let fa = await store.get('fileauth');
    fileauth.value = fa ? fa+'' : 'R'

    let t: any = await store.get('tableData');
    console.log("tableData:" + t)
    try {
        if (t && t.length > 0 && typeof t != 'string') {
            tableData.value = t
        } else {
            tableData.value = []
        }
    } catch (e) {
        console.log(e)
        tableData.value = []
        store.set('tableData', []);
    }
}

onMounted(() => {
    init()
})
const form = reactive({ username: '', password: '', index: null, fileauth: 'R' })
const drawer = ref(false)
const direction = ref<DrawerProps['direction']>('btt')

const deleteRow = (scope: any) => {
    tableData.value.splice(scope.$index, 1)
    store.set('tableData', tableData.value)
}
const saveForm = () => {
    if (form.password && form.password.length > 0 && form.username && form.username.length > 0) {
        drawer.value = !drawer
        console.log(form)
        console.log(tableData.value)
        if (form.index != null) {
            tableData.value[form.index] = Object.assign({}, form)
        } else {
            tableData.value[tableData.value.length] = Object.assign({}, form)
        }
        console.log(tableData.value)
        store.set('tableData', tableData.value)
    } else {
        ElMessage.error('请输入用户名和密码')
    }
}

</script>

<style scoped></style>