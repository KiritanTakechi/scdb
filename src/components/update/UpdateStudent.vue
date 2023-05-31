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
import { FormRules } from 'element-plus';
import { reactive, ref } from 'vue';

interface Student {
    f_sno: string | null;
    f_name: string | null;
    f_sex: string;
    f_age: number | null;
    f_dept: string | null;
}

const studentData = reactive<Student>({
    f_sno: null,
    f_name: null,
    f_sex: '',
    f_age: null,
    f_dept: null
});

const rules = reactive<FormRules>({

});

const onSubmit = () => {

}

const clear = () => {
    studentData.f_sno = null;
    studentData.f_name = null;
    studentData.f_sex = '';
    studentData.f_age = null;
    studentData.f_dept = null;
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