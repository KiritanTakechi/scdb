<template>
    <div class="student-list">
        <el-table height="100%" width="100%" stripe :data="tableData">
            <el-table-column prop="sno" label="学号" />
            <el-table-column prop="name" label="姓名" />
            <el-table-column prop="sex" label="性别" />
            <el-table-column prop="age" label="年龄" />
            <el-table-column prop="dept" label="学院" />
        </el-table>
    </div>
</template>
    
<script setup lang='ts'>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const tableData = [
    {
        sno: 1,
        name: '丁真',
        sex: '男',
        age: 20,
        dept: '理塘学院',
    },
]

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