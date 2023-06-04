<template>
    <div class="update-course">
        <el-card class="update-course__card">
            <template #header>
                <div class="card-header">
                    <span>课程信息：</span>
                </div>
            </template>
            <el-form :label-position="left" label-width="70px" :model="courseData" :rules="rules" style="max-width: 90%">
                <el-form-item label="课程号">
                    <el-input v-model="courseData.f_cno" />
                </el-form-item>
                <el-form-item label="课程名">
                    <el-input v-model="courseData.f_name" />
                </el-form-item>
                <el-form-item label="学分">
                    <el-input v-model="courseData.f_credit" />
                </el-form-item>
                <el-form-item label="所在学期">
                    <el-input v-model="courseData.f_semester" />
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
import { FormRules } from 'element-plus';
import { reactive, ref } from 'vue';


type Course = {
    f_cno: string,
    f_name: string,
    f_credit: number,
    f_semester: number,
}

const courseData = ref<Course>({
    f_cno: '',
    f_name: '',
    f_credit: 0,
    f_semester: 0,
});

const rules = reactive<FormRules>({

});

const onSubmit = async () => {
    let _ = await invoke("course_insert", { course: courseData.value });
}

const clear = () => {
    courseData.value.f_cno = '';
    courseData.value.f_name = '';
    courseData.value.f_credit = 0;
    courseData.value.f_semester = 0;
}
</script>
    
<style scoped>
.update-course {
    height: 100%;
    width: 100%;

    background-color: whitesmoke;

    display: flex;
    justify-content: center;
    align-items: center;
}
</style>