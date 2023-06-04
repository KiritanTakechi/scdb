<template>
    <div class="update-student">
        <el-card class="update-student__card">
            <template #header>
                <div class="card-header">
                    <span>学生信息：</span>
                </div>
            </template>
            <el-form :label-position="left" label-width="40px" :model="studentData" :rules="rules" style="max-width: 90%">
                <el-form-item label="学号">
                    <el-input v-model="studentData.f_sno" />
                </el-form-item>
                <el-form-item label="姓名">
                    <el-input v-model="studentData.f_name" />
                </el-form-item>
                <el-form-item label="性别">
                    <el-select v-model="studentData.f_sex" placeholder="选择你的性别">
                        <el-option label="男" value="男" />
                        <el-option label="女" value="女" />
                    </el-select>
                </el-form-item>
                <el-form-item label="年龄">
                    <el-input v-model="studentData.f_age" />
                </el-form-item>
                <el-form-item label="学院">
                    <el-input v-model="studentData.f_dept" />
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click.prevent="onSubmit">添加</el-button>
                    <el-button @click.prevent="clear">清空</el-button>
                </el-form-item>
            </el-form>
        </el-card>
    </div>
</template>
    
<script setup lang='ts'>
import { left } from '@popperjs/core';
import { invoke } from '@tauri-apps/api/tauri';
import { FormInstance, FormRules } from 'element-plus';
import { reactive, ref } from 'vue';

interface Student {
    f_sno: string
    f_name: string;
    f_sex: string;
    f_age: number;
    f_dept: string;
}

const studentData = ref<Student>({
    f_sno: '',
    f_name: '',
    f_sex: '',
    f_age: 18,
    f_dept: ''
});

/* 未实现 */
const rules = reactive<FormRules>({
    f_sno: [
        { required: true, message: '请输入学号', trigger: 'blur' },
        { min: 1, max: 10, message: '输入1到10位学号', trigger: 'blur' },
    ],
    f_name: [
        { required: true, message: '请输入姓名', trigger: 'blur' },
    ],
    f_sex: [
        { required: true, message: '请选择性别', trigger: 'change' },
    ],
    f_age: [
        { required: true, message: '请输入年龄', trigger: 'blur' },
        { min: 0, max: 120, message: '请输入正确的年龄', trigger: 'blur' },
    ],
    f_dept: [
        { required: true, message: '请输入学院', trigger: 'blur' },
    ]
});

const onSubmit = async () => {
    let _ = await invoke("student_insert", { student: studentData.value });
}

const clear = () => {
    studentData.value.f_sno = '';
    studentData.value.f_name = '';
    studentData.value.f_sex = '';
    studentData.value.f_age = 18;
    studentData.value.f_dept = '';
}
</script>
    
<style scoped>
.update-student {
    height: 100%;
    width: 100%;

    background-color: whitesmoke;

    display: flex;
    justify-content: center;
    align-items: center;
}
</style>