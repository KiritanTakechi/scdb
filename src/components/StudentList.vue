<template>
    <div class="student-list">
        <el-table height="100%" width="100%" stripe :data="studentData">
            <el-table-column prop="f_sno" label="学号" />
            <el-table-column prop="f_name" label="姓名" />
            <el-table-column prop="f_sex" label="性别" />
            <el-table-column prop="f_age" label="年龄" />
            <el-table-column prop="f_dept" label="学院" />
        </el-table>
    </div>
</template>
    
<script setup lang='ts'>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface Student {
    f_sno: string;
    f_name: string;
    f_sex: string;
    f_age: number;
    f_dept: string;
}

const studentData = ref<Student[]>([]);

onMounted(async () => {
    studentData.value = await invoke("student_read_all");
});
</script>
    
<style scoped>
.student-list {
    height: 100%;
    width: 100%;

    background-color: whitesmoke;

    display: flex;
    justify-content: center;
    align-items: flex-start;
}
</style>