<template>
    <div class="course-list">
        <el-table height="100%" width="100%" stripe :data="tableData">
            <el-table-column prop="cno" label="课程号" />
            <el-table-column prop="name" label="课程名" />
            <el-table-column prop="credit" label="学分" />
            <el-table-column prop="semester" label="所在学期" />
        </el-table>
    </div>
</template>
    
<script setup lang='ts'>
const tableData = [
    {
        cno: 1,
        name: '数据库',
        credit: 3,
        semester: 2,
    },
]
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