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
import { FormRules } from 'element-plus';
import { reactive, ref } from 'vue';


type Course = {
    f_cno: string | null,
    f_name: string | null,
    f_credit: number | null,
    f_semester: number | null,
}

const courseData = reactive<Course>({
    f_cno: null,
    f_name: null,
    f_credit: null,
    f_semester: null,
});

const rules = reactive<FormRules>({

});

const onSubmit = () => {

}

const clear = () => {
    courseData.f_cno = null;
    courseData.f_name = null;
    courseData.f_credit = null;
    courseData.f_semester = null;
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