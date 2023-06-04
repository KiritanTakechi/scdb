<template>
    <div class="update-sc">
        <el-card class="update-sc__card">
            <template #header>
                <div class="card-header">
                    <span>学生选课信息：</span>
                </div>
            </template>
            <el-form :label-position="left" label-width="70px" :model="scData" :rules="rules" style="max-width: 90%">
                <el-form-item label="学生号">
                    <el-input v-model="scData.f_sno" />
                </el-form-item>
                <el-form-item label="课程号">
                    <el-input v-model="scData.f_cno" />
                </el-form-item>
                <el-form-item label="成绩">
                    <el-input v-model="scData.f_grade" />
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


type SC = {
    f_sno: string;
    f_cno: string;
    f_grade: number;
}

const scData = ref<SC>({
    f_sno: '',
    f_cno: '',
    f_grade: 0,
});

const rules = reactive<FormRules>({

});

const onSubmit = async () => {
    let _ = await invoke("sc_insert", { sc: scData.value });
}

const clear = () => {
    scData.value.f_sno = '';
    scData.value.f_cno = '';
    scData.value.f_grade = 0;
}
</script>
    
<style scoped>
.update-sc {
    height: 100%;
    width: 100%;

    background-color: whitesmoke;

    display: flex;
    justify-content: center;
    align-items: center;
}
</style>