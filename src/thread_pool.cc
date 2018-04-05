#include "thread_pool.h"  
#include <iostream>  

void CCountTask::SetData(void * data)
{
	m_ptrData = data;
}

vector<CCountTask*> CThreadPool::_vecTaskList;         //任务列表  
bool CThreadPool::_shutdown = false;

pthread_mutex_t CThreadPool::_pthreadMutex = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t CThreadPool::_pthreadCond = PTHREAD_COND_INITIALIZER;

/**
* 线程池管理类构造函数
*/
CThreadPool::CThreadPool(int threadNum)
{
	this->_iThreadNum = threadNum;
	Create();
}

/**
* 线程回调函数
*/
void* CThreadPool::ThreadFunc(void* threadData)
{
	pthread_t tid = pthread_self();
	while (1)
	{
		pthread_mutex_lock(&_pthreadMutex);
		while (_vecTaskList.size() == 0 && !_shutdown)
		{
			pthread_cond_wait(&_pthreadCond, &_pthreadMutex);
		}

		if (_shutdown)
		{
			pthread_mutex_unlock(&_pthreadMutex);
			pthread_exit(NULL);
		}

		vector<CCountTask*>::iterator iter = _vecTaskList.begin();

		/**
		* 取出一个任务并处理之
		*/
		CCountTask* task = *iter;
		if (iter != _vecTaskList.end())
		{
			task = *iter;
			_vecTaskList.erase(iter);
		}

		pthread_mutex_unlock(&_pthreadMutex);

		task->count(); /** 执行任务 */
	}
	return (void*)0;
}

/**
* 往任务队列里边添加任务并发出线程同步信号
*/
int CThreadPool::AddTask(CCountTask *task)
{
	pthread_mutex_lock(&_pthreadMutex);
	this->_vecTaskList.push_back(task);
	pthread_mutex_unlock(&_pthreadMutex);
	pthread_cond_signal(&_pthreadCond);
	return 0;
}

/**
* 创建线程
*/
int CThreadPool::Create()
{
	_pthread_id = (pthread_t*)malloc(sizeof(pthread_t)* _iThreadNum);
	for (int i = 0; i < _iThreadNum; i++)
	{
		pthread_create(&_pthread_id[i], NULL, ThreadFunc, NULL);
	}
	return 0;
}

/**
* 停止所有线程
*/
int CThreadPool::StopAll()
{
	if (_shutdown)
	{
		return -1;
	}

	_shutdown = true;
	pthread_cond_broadcast(&_pthreadCond);

	for (int i = 0; i < _iThreadNum; i++)
	{
		pthread_join(_pthread_id[i], NULL);
	}

	free(_pthread_id);
	_pthread_id = NULL;

	pthread_mutex_destroy(&_pthreadMutex);
	pthread_cond_destroy(&_pthreadCond);

	return 0;
}

/**
* 获取当前队列中任务数
*/
int CThreadPool::getTaskSize()
{
	return _vecTaskList.size();
}
