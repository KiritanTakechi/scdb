<template>
    <div class="course-list">
        <el-table height="100%" width="100%" stripe :data="courseData">
            <el-table-column prop="f_cno" label="课程号" />
            <el-table-column prop="f_name" label="课程名" />
            <el-table-column prop="f_credit" label="学分" />
            <el-table-column prop="f_semester" label="所在学期" />
        </el-table>
    </div>
</template>
    
<script setup lang='ts'>
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, ref } from 'vue';


type Course = {
    f_cno: string,
    f_name: string,
    f_credit: number,
    f_semester: number,
}

const courseData = ref<Course[]>([]);

onMounted(async () => {
    courseData.value = await invoke("course_read_all");
});
</script>
    
<style scoped>
.course-list {
    height: 100%;
    width: 100%;

    background-color: whitesmoke;

    display: flex;
    justify-content: center;
    align-items: flex-start;
}
</style>